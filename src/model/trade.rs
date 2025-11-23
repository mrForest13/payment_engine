use crate::model::client::ClientId;
use rust_decimal::Decimal;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Deserialize)]
pub struct TransactionId(pub u32);

impl fmt::Display for TransactionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Transaction {
    Deposit {
        client: ClientId,
        trade: TransactionId,
        amount: Decimal,
    },
    Withdrawal {
        client: ClientId,
        trade: TransactionId,
        amount: Decimal,
    },
    Dispute {
        client: ClientId,
        trade: TransactionId,
    },
    Resolve {
        client: ClientId,
        trade: TransactionId,
    },
    Chargeback {
        client: ClientId,
        trade: TransactionId,
    },
}
