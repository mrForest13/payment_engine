use payment_engine::errors::{EngineError, EngineResult};
use std::env;
use tracing::info;

#[tokio::main]
async fn main() -> EngineResult<()> {
    tracing_subscriber::fmt::init();

    let file = get_file_path()?;

    info!("Fetching {} file...", file);

    Ok(())
}

fn get_file_path() -> EngineResult<String> {
    if let Some(file) = env::args().nth(1) {
        Ok(file)
    } else {
        Err(EngineError::InputNotProvided())
    }
}
