use payment_engine::core::wallet::AccountWallet;
use payment_engine::errors::EngineError;
use payment_engine::model::account::Account;
use payment_engine::model::client::ClientId;
use payment_engine::model::trade::TransactionId;
use rust_decimal_macros::dec;

#[test]
fn deposit_negative_amount() -> anyhow::Result<()> {
    let mut wallet = AccountWallet::new(ClientId(1));

    let trade = TransactionId(1);
    let amount = dec!(-1.5);

    let confirmation = wallet.deposit(trade, amount);

    assert!(confirmation.is_err());
    assert_eq!(confirmation, Err(EngineError::NegativeAmount(trade)));

    Ok(())
}

#[test]
fn deposit_zero_amount() -> anyhow::Result<()> {
    let mut wallet = AccountWallet::new(ClientId(1));

    let trade = TransactionId(1);
    let amount = dec!(0.0);

    let confirmation = wallet.deposit(trade, amount);

    assert!(confirmation.is_err());
    assert_eq!(confirmation, Err(EngineError::NegativeAmount(trade)));

    Ok(())
}

#[test]
fn deposit_positive_amount() -> anyhow::Result<()> {
    let mut wallet = AccountWallet::new(ClientId(1));

    let trade = TransactionId(1);
    let amount = dec!(1.5);

    let confirmation = wallet.deposit(trade, amount);

    assert!(confirmation.is_ok());

    let trade = TransactionId(2);
    let amount = dec!(2.0);

    let confirmation = wallet.deposit(trade, amount);

    assert!(confirmation.is_ok());

    let account: Account = wallet.into();

    assert_eq!(account.client, ClientId(1));
    assert_eq!(account.available, dec!(3.5));
    assert_eq!(account.held, dec!(0));
    assert_eq!(account.total, dec!(3.5));
    assert_eq!(account.locked, false);

    Ok(())
}
