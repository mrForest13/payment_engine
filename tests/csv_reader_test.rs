use payment_engine::input::csv::CsvReader;
use payment_engine::input::reader::InputReader;
use payment_engine::model::client::ClientId;
use payment_engine::model::trade::{Transaction, TransactionId};
use rust_decimal::Decimal;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn read_all_transaction_types_from_file() -> anyhow::Result<()> {
    let mut file = NamedTempFile::new()?;

    writeln!(file, "type,client,tx,amount")?;
    writeln!(file, "deposit,1,1,1.0")?;
    writeln!(file, "withdrawal,1,4,1.5")?;
    writeln!(file, "dispute,1,1,")?;
    writeln!(file, "resolve,1,1,")?;
    writeln!(file, "chargeback,1,1,")?;

    let mut reader = CsvReader::from_file(file.reopen()?)?;

    let deposit = reader.next().unwrap()?;
    assert_eq!(
        deposit,
        Transaction::Deposit {
            client: ClientId(1),
            trade: TransactionId(1),
            amount: Decimal::new(1, 0)
        }
    );

    let withdrawal = reader.next().unwrap()?;
    assert_eq!(
        withdrawal,
        Transaction::Withdrawal {
            client: ClientId(1),
            trade: TransactionId(4),
            amount: Decimal::new(15, 1)
        }
    );

    let dispute = reader.next().unwrap()?;
    assert_eq!(
        dispute,
        Transaction::Dispute {
            client: ClientId(1),
            trade: TransactionId(1),
        }
    );

    let resolve = reader.next().unwrap()?;
    assert_eq!(
        resolve,
        Transaction::Resolve {
            client: ClientId(1),
            trade: TransactionId(1),
        }
    );

    let chargeback = reader.next().unwrap()?;
    assert_eq!(
        chargeback,
        Transaction::Chargeback {
            client: ClientId(1),
            trade: TransactionId(1),
        }
    );

    Ok(())
}

#[test]
fn read_csv_files_with_whitespaces() -> anyhow::Result<()> {
    let mut file = NamedTempFile::new()?;

    writeln!(file, "type , client , tx , amount ")?;
    writeln!(file, " deposit , 1 , 1, 1.0 ")?;

    let mut reader = CsvReader::from_file(file.reopen()?)?;

    let deposit = reader.next().unwrap()?;
    assert_eq!(
        deposit,
        Transaction::Deposit {
            client: ClientId(1),
            trade: TransactionId(1),
            amount: Decimal::new(1, 0)
        }
    );

    Ok(())
}
