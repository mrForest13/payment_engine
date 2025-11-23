use crate::errors::{EngineError, EngineResult};
use crate::model::client::ClientId;
use crate::model::trade::{Transaction, TransactionId};
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, Deserialize)]
pub struct TransactionRow {
    r#type: TransactionType,
    client: ClientId,
    tx: TransactionId,
    amount: Option<Decimal>,
}

impl TransactionRow {
    fn get_amount(&self) -> EngineResult<Decimal> {
        self.amount.ok_or(EngineError::MissingAmount())
    }
}

impl TryFrom<TransactionRow> for Transaction {
    type Error = EngineError;

    fn try_from(row: TransactionRow) -> Result<Self, Self::Error> {
        match row.r#type {
            TransactionType::Deposit => Ok(Transaction::Deposit {
                client: row.client,
                trade: row.tx,
                amount: row.get_amount()?,
            }),
            TransactionType::Withdrawal => Ok(Transaction::Withdrawal {
                client: row.client,
                trade: row.tx,
                amount: row.get_amount()?,
            }),
            TransactionType::Dispute => Ok(Transaction::Dispute {
                client: row.client,
                trade: row.tx,
            }),
            TransactionType::Resolve => Ok(Transaction::Resolve {
                client: row.client,
                trade: row.tx,
            }),
            TransactionType::Chargeback => Ok(Transaction::Chargeback {
                client: row.client,
                trade: row.tx,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::errors::{EngineError, EngineResult};
    use crate::input::row::{TransactionRow, TransactionType};
    use crate::model::client::ClientId;
    use crate::model::trade::{Transaction, TransactionId};
    use rust_decimal_macros::dec;

    #[test]
    fn test_valid_from_for_deposit() -> EngineResult<()> {
        let row = TransactionRow {
            r#type: TransactionType::Deposit,
            client: ClientId(1),
            tx: TransactionId(2),
            amount: Some(dec!(1.5)),
        };

        let dto: EngineResult<Transaction> = row.try_into();

        assert!(dto.is_ok());
        assert_eq!(
            dto?,
            Transaction::Deposit {
                client: ClientId(1),
                trade: TransactionId(2),
                amount: dec!(1.5),
            }
        );

        Ok(())
    }

    #[test]
    fn test_invalid_from_for_deposit() -> EngineResult<()> {
        let row = TransactionRow {
            r#type: TransactionType::Deposit,
            client: ClientId(1),
            tx: TransactionId(2),
            amount: None,
        };

        let dto: EngineResult<Transaction> = row.try_into();

        assert!(dto.is_err());
        assert_eq!(dto, Err(EngineError::MissingAmount()));

        Ok(())
    }

    #[test]
    fn test_valid_from_for_withdrawal() -> EngineResult<()> {
        let row = TransactionRow {
            r#type: TransactionType::Withdrawal,
            client: ClientId(1),
            tx: TransactionId(2),
            amount: Some(dec!(1.5)),
        };

        let dto: EngineResult<Transaction> = row.try_into();

        assert!(dto.is_ok());
        assert_eq!(
            dto?,
            Transaction::Withdrawal {
                client: ClientId(1),
                trade: TransactionId(2),
                amount: dec!(1.5),
            }
        );

        Ok(())
    }

    #[test]
    fn test_invalid_from_for_withdrawal() -> EngineResult<()> {
        let row = TransactionRow {
            r#type: TransactionType::Withdrawal,
            client: ClientId(1),
            tx: TransactionId(2),
            amount: None,
        };

        let dto: EngineResult<Transaction> = row.try_into();

        assert!(dto.is_err());
        assert_eq!(dto, Err(EngineError::MissingAmount()));

        Ok(())
    }

    #[test]
    fn test_valid_from_for_dispute() -> EngineResult<()> {
        let row = TransactionRow {
            r#type: TransactionType::Dispute,
            client: ClientId(1),
            tx: TransactionId(2),
            amount: None,
        };

        let dto: EngineResult<Transaction> = row.try_into();

        assert!(dto.is_ok());
        assert_eq!(
            dto?,
            Transaction::Dispute {
                client: ClientId(1),
                trade: TransactionId(2),
            }
        );

        Ok(())
    }

    #[test]
    fn test_valid_from_for_resolve() -> EngineResult<()> {
        let row = TransactionRow {
            r#type: TransactionType::Resolve,
            client: ClientId(1),
            tx: TransactionId(2),
            amount: None,
        };

        let dto: EngineResult<Transaction> = row.try_into();

        assert!(dto.is_ok());
        assert_eq!(
            dto?,
            Transaction::Resolve {
                client: ClientId(1),
                trade: TransactionId(2),
            }
        );

        Ok(())
    }

    #[test]
    fn test_valid_from_for_chargeback() -> EngineResult<()> {
        let row = TransactionRow {
            r#type: TransactionType::Chargeback,
            client: ClientId(1),
            tx: TransactionId(2),
            amount: None,
        };

        let dto: EngineResult<Transaction> = row.try_into();

        assert!(dto.is_ok());
        assert_eq!(
            dto?,
            Transaction::Chargeback {
                client: ClientId(1),
                trade: TransactionId(2),
            }
        );

        Ok(())
    }
}
