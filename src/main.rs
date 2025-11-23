use payment_engine::core::engine::PaymentEngine;
use payment_engine::errors::{EngineError, EngineResult};
use payment_engine::input::csv::CsvReader;
use payment_engine::input::reader::InputReader;
use std::env;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> EngineResult<()> {
    // tracing_subscriber::fmt::init();

    let file = get_file_path()?;

    info!("Fetching {} file...", file);

    let mut reader = CsvReader::new(&file)?;
    let mut engine = PaymentEngine::default();

    while let Some(result) = reader.next() {
        match result {
            Ok(transaction) => {
                engine.process(transaction).await?;
            }
            Err(error) => {
                warn!(?error, "Cannot deserialize transaction");
            }
        }
    }

    println!("{}", engine.report().await?);

    Ok(())
}

fn get_file_path() -> EngineResult<String> {
    if let Some(file) = env::args().nth(1) {
        Ok(file)
    } else {
        Err(EngineError::InputNotProvided())
    }
}
