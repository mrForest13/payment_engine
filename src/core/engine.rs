use crate::core::wallet::AccountWallet;
use crate::core::worker::EngineWorker;
use crate::errors::{EngineError, EngineResult};
use crate::model::account::Account;
use crate::model::client::ClientId;
use crate::model::report::Report;
use crate::model::trade::Transaction;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tracing::{info, warn};

const DEFAULT_WORKERS_SIZE: usize = 10;
const DEFAULT_BUFFER_SIZE: usize = 100;

type Wallets = HashMap<ClientId, AccountWallet>;

pub struct PaymentEngine {
    workers_size: u16,
    worker_buffer: usize,
    workers: HashMap<usize, (mpsc::Sender<Transaction>, JoinHandle<Wallets>)>,
}

impl Default for PaymentEngine {
    fn default() -> Self {
        Self::new(DEFAULT_WORKERS_SIZE)
    }
}

impl PaymentEngine {
    pub fn new(pool_size: usize) -> PaymentEngine {
        Self {
            workers_size: pool_size as u16,
            worker_buffer: DEFAULT_BUFFER_SIZE,
            workers: HashMap::with_capacity(pool_size),
        }
    }

    pub async fn report(mut self) -> Result<Report, EngineError> {
        let handlers: Vec<_> = self
            .workers
            .drain()
            .map(|(_, (_, handler))| handler)
            .collect();

        let mut accounts = vec![];

        for handler in handlers {
            let result: Vec<Account> = handler
                .await
                .map_err(|_| EngineError::InternalError())?
                .drain()
                .map(|(_, wallet)| wallet.into())
                .collect();
            accounts.extend(result);
        }

        Ok(Report::new(accounts))
    }

    pub async fn process(&mut self, tx: Transaction) -> EngineResult<()> {
        let id = self.worker_id(tx.client_id()) as usize;

        let (worker, _) = self
            .workers
            .entry(id)
            .or_insert_with(|| init_worker(id, self.worker_buffer));

        worker
            .send(tx)
            .await
            .map_err(|_| EngineError::InternalError())
    }

    fn worker_id(&self, id: ClientId) -> u16 {
        id.0 % self.workers_size
    }
}

fn init_worker(id: usize, buffer: usize) -> (mpsc::Sender<Transaction>, JoinHandle<Wallets>) {
    let (tx, mut rx): (mpsc::Sender<Transaction>, mpsc::Receiver<Transaction>) =
        mpsc::channel::<Transaction>(buffer);

    let accounts = tokio::spawn(async move {
        info!("Initialize worker with id {}", id);

        let mut worker = EngineWorker::new(id);

        while let Some(tx) = rx.recv().await {
            info!("Processing transaction by worker {}", worker.id);

            worker.handle(tx).unwrap_or_else(|error| {
                warn!("Transaction has been rejected: {:?}", error);
            });
        }

        worker.accounts()
    });

    (tx, accounts)
}
