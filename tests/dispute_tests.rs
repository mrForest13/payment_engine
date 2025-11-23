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

    Ok(wallet)
}

#[test]
fn dispute_unexist_transaction() -> anyhow::Result<()> {
    let mut wallet = init_wallet(ClientId(1))?;

    let trade = TransactionId(3);

    let confirmation = wallet.dispute(trade);

    assert!(confirmation.is_err());
    assert_eq!(confirmation, Err(EngineError::TransactionNotFound(trade)));

    Ok(())
}

#[test]
fn dispute_more_than_account_has() -> anyhow::Result<()> {
    let mut wallet = init_wallet(ClientId(1))?;

    let trade = TransactionId(3);
    let amount = dec!(6);

    wallet.withdrawal(trade, amount)?;

    let trade = TransactionId(2);

    let confirmation = wallet.dispute(trade);

    assert!(confirmation.is_err());
    assert_eq!(confirmation, Err(EngineError::NotEnoughMany(trade)));

    Ok(())
}

#[test]
fn dispute_positive_amount() -> anyhow::Result<()> {
    let mut wallet = init_wallet(ClientId(1))?;

    let trade = TransactionId(2);

    let confirmation = wallet.dispute(trade);

    assert!(confirmation.is_ok());

    let account: Account = wallet.into();

    assert_eq!(account.client, ClientId(1));
    assert_eq!(account.available, dec!(2));
    assert_eq!(account.held, dec!(5));
    assert_eq!(account.total, dec!(7));
    assert_eq!(account.locked, false);

    Ok(())
}
