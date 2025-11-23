use payment_engine::core::wallet::AccountWallet;
use payment_engine::errors::EngineError;
use payment_engine::model::account::Account;
use payment_engine::model::client::ClientId;
use payment_engine::model::trade::TransactionId;
use rust_decimal_macros::dec;

fn init_wallet(client_id: ClientId) -> anyhow::Result<AccountWallet> {
    let mut wallet = AccountWallet::new(client_id);

    let trade = TransactionId(1);
    let amount = dec!(2);
    wallet.deposit(trade, amount)?;

    let trade = TransactionId(2);
    let amount = dec!(5);
    wallet.deposit(trade, amount)?;

    let trade = TransactionId(2);
    wallet.dispute(trade)?;

    Ok(wallet)
}

#[test]
fn chargeback_unexist_dispute() -> anyhow::Result<()> {
    let mut wallet = init_wallet(ClientId(1))?;

    let trade = TransactionId(1);

    let confirmation = wallet.chargeback(trade);

    assert!(confirmation.is_err());
    assert_eq!(confirmation, Err(EngineError::TransactionNotFound(trade)));

    Ok(())
}

#[test]
fn chargeback_positive_amount() -> anyhow::Result<()> {
    let mut wallet = init_wallet(ClientId(1))?;

    let trade = TransactionId(2);

    let confirmation = wallet.chargeback(trade);

    assert!(confirmation.is_ok());

    let account: Account = wallet.into();

    assert_eq!(account.client, ClientId(1));
    assert_eq!(account.available, dec!(2));
    assert_eq!(account.held, dec!(0));
    assert_eq!(account.total, dec!(2));
    assert_eq!(account.locked, true);

    Ok(())
}

#[test]
fn deposit_locked_account() -> anyhow::Result<()> {
    let mut wallet = init_wallet(ClientId(1))?;

    let trade = TransactionId(2);

    let confirmation = wallet.chargeback(trade);

    assert!(confirmation.is_ok());

    let trade = TransactionId(3);
    let amount = dec!(1.5);

    let confirmation = wallet.deposit(trade, amount);

    assert!(confirmation.is_err());
    assert_eq!(confirmation, Err(EngineError::FrozenAccount(ClientId(1))));

    Ok(())
}
