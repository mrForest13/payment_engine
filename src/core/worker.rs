use crate::core::wallet::AccountWallet;
use crate::errors::EngineResult;
use crate::model::client::ClientId;
use crate::model::trade::Transaction;
use std::collections::HashMap;

pub struct EngineWorker {
    pub id: usize,
    accounts: HashMap<ClientId, AccountWallet>,
}

impl EngineWorker {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            accounts: HashMap::new(),
        }
    }

    pub fn accounts(self) -> HashMap<ClientId, AccountWallet> {
        self.accounts
    }

    pub fn handle(&mut self, trade: Transaction) -> EngineResult<()> {
        let account = self.get_account(trade.client_id());

        match trade {
            Transaction::Deposit {
                client: _,
                trade,
                amount,
            } => account.deposit(trade, amount),
            Transaction::Withdrawal {
                client: _,
                trade,
                amount,
            } => account.withdrawal(trade, amount),
            Transaction::Dispute { client: _, trade } => account.dispute(trade),
            Transaction::Resolve { client: _, trade } => account.resolve(trade),
            Transaction::Chargeback { client: _, trade } => account.chargeback(trade),
        }
    }

    fn get_account(&mut self, client_id: ClientId) -> &mut AccountWallet {
        self.accounts
            .entry(client_id)
            .or_insert_with(|| AccountWallet::new(client_id))
    }
}
