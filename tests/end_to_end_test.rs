use indoc::indoc;
use payment_engine::core::engine::PaymentEngine;
use payment_engine::input::csv::CsvReader;
use payment_engine::input::reader::InputReader;

#[tokio::test]
async fn end_to_end_test_based_on_csv_file() -> anyhow::Result<()> {
    let file = "transactions.csv";

    let mut reader = CsvReader::new(&file)?;
    let mut engine = PaymentEngine::default();

    while let Some(result) = reader.next() {
        match result {
            Ok(transaction) => {
                engine.process(transaction).await?;
            }
            Err(error) => {
                return Err(error.into());
            }
        }
    }

    let report = engine.report().await?;

    let expected = indoc! {r#"
        client,available,held,total,locked
        2,2,0,2,false
        1,0.5,0,0.5,true
    "#};

    assert_eq!(report.to_string(), expected);

    Ok(())
}
