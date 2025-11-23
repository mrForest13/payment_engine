use crate::errors::{EngineError, EngineResult};
use crate::model::account::Account;
use crate::model::client::ClientId;
use crate::model::trade::TransactionId;
use rust_decimal::Decimal;
use std::collections::HashMap;

pub struct AccountWallet {
    client: ClientId,
    available: Decimal,
    held: Decimal,
    total: Decimal,
    locked: bool,
    deposits: HashMap<TransactionId, Decimal>,
    disputes: HashMap<TransactionId, Decimal>,
}

impl From<AccountWallet> for Account {
    fn from(wallet: AccountWallet) -> Self {
        Account {
            client: wallet.client,
            available: wallet.available,
            held: wallet.held,
            total: wallet.total,
            locked: wallet.locked,
        }
    }
}

impl AccountWallet {
    pub fn new(client_id: ClientId) -> Self {
        Self {
            client: client_id,
            available: Decimal::ZERO,
            held: Decimal::ZERO,
            total: Decimal::ZERO,
            locked: false,
            deposits: HashMap::new(),
            disputes: HashMap::new(),
        }
    }

    fn check_frozen(&self) -> EngineResult<()> {
        if self.locked {
            Err(EngineError::FrozenAccount(self.client))
        } else {
            Ok(())
        }
    }

    fn check_available_founds(&self, id: TransactionId, amount: &Decimal) -> EngineResult<()> {
        if self.available < *amount {
            Err(EngineError::NotEnoughMany(id))
        } else {
            Ok(())
        }
    }

    fn check_held_founds(&self, id: TransactionId, amount: &Decimal) -> EngineResult<()> {
        if self.held < *amount {
            Err(EngineError::NotEnoughMany(id))
        } else {
            Ok(())
        }
    }

    fn find_deposit(&self, id: TransactionId) -> EngineResult<Decimal> {
        self.deposits
            .get(&id)
            .cloned()
            .ok_or(EngineError::TransactionNotFound(id))
    }

    fn find_dispute(&self, id: TransactionId) -> EngineResult<Decimal> {
        self.disputes
            .get(&id)
            .cloned()
            .ok_or(EngineError::TransactionNotFound(id))
    }

    pub fn deposit(&mut self, id: TransactionId, amount: Decimal) -> EngineResult<()> {
        let amount = validate_amount(id, amount)?;

        self.check_frozen()?;

        self.available += amount;
        self.total += amount;

        self.deposits.insert(id, amount);

        Ok(())
    }

    pub fn withdrawal(&mut self, id: TransactionId, amount: Decimal) -> EngineResult<()> {
        let amount = validate_amount(id, amount)?;

        self.check_frozen()?;
        self.check_available_founds(id, &amount)?;

        self.available -= amount;
        self.total -= amount;

        Ok(())
    }

    pub fn dispute(&mut self, id: TransactionId) -> EngineResult<()> {
        let amount = self.find_deposit(id)?;

        self.check_frozen()?;
        self.check_available_founds(id, &amount)?;

        self.available -= amount;
        self.held += amount;

        self.disputes.insert(id, amount);

        Ok(())
    }

    pub fn resolve(&mut self, id: TransactionId) -> EngineResult<()> {
        let amount = self.find_dispute(id)?;

        self.check_frozen()?;
        self.check_held_founds(id, &amount)?;

        self.available += amount;
        self.held -= amount;

        self.disputes.remove(&id);

        Ok(())
    }

    pub fn chargeback(&mut self, id: TransactionId) -> EngineResult<()> {
        let amount = self.find_dispute(id)?;

        self.check_frozen()?;
        self.check_held_founds(id, &amount)?;

        self.total -= amount;
        self.held -= amount;
        self.locked = true;

        self.disputes.remove(&id);

        Ok(())
    }
}

fn validate_amount(id: TransactionId, amount: Decimal) -> EngineResult<Decimal> {
    if amount.is_sign_negative() || amount.is_zero() {
        Err(EngineError::NegativeAmount(id))
    } else if amount.scale() > 4 {
        Err(EngineError::InvalidPrecision(id))
    } else {
        Ok(amount)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::wallet::AccountWallet;
    use crate::errors::EngineError;
    use crate::model::client::ClientId;
    use crate::model::trade::TransactionId;
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    #[test]
    fn test_negative_transaction_amount() {
        let mut account = AccountWallet::new(ClientId(1));

        let trade_id = TransactionId(1);
        let amount = dec!(-100.0);

        let result = account.deposit(trade_id, amount);

        assert!(result.is_err());

        if let Err(error) = result {
            assert_eq!(error, EngineError::NegativeAmount(trade_id));
        }
    }

    #[test]
    fn test_zero_transaction_amount() {
        let mut account = AccountWallet::new(ClientId(1));

        let trade_id = TransactionId(1);
        let amount = Decimal::ZERO;

        let result = account.deposit(trade_id, amount);

        assert!(result.is_err());

        if let Err(error) = result {
            assert_eq!(error, EngineError::NegativeAmount(trade_id));
        }
    }

    #[test]
    fn test_invalid_precision_transaction() {
        let mut account = AccountWallet::new(ClientId(1));

        let trade_id = TransactionId(1);
        let amount = dec!(1.12345);

        let result = account.deposit(trade_id, amount);

        assert!(result.is_err());

        if let Err(error) = result {
            assert_eq!(error, EngineError::InvalidPrecision(trade_id));
        }
    }

    #[test]
    fn test_positive_transaction_amount() {
        let mut account = AccountWallet::new(ClientId(1));

        let trade_id = TransactionId(1);
        let amount = dec!(1.1234);

        let result = account.deposit(trade_id, amount);

        assert!(result.is_ok());
    }
}
