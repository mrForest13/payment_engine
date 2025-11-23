use crate::model::client::ClientId;
use rust_decimal::Decimal;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct TransactionId(pub u32);

impl fmt::Display for TransactionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub enum Transaction {
    Deposit {
        client: ClientId,
        tx: TransactionId,
        amount: Decimal,
    },
    Withdrawal {
        client: ClientId,
        tx: TransactionId,
        amount: Decimal,
    },
    Dispute {
        client: ClientId,
        tx: TransactionId,
    },
    Resolve {
        client: ClientId,
        tx: TransactionId,
    },
    Chargeback {
        client: ClientId,
        tx: TransactionId,
    },
}
