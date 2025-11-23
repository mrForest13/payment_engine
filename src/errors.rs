use crate::model::client::ClientId;
use crate::model::trade::TransactionId;
use thiserror::Error;

pub type EngineResult<T> = Result<T, EngineError>;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum EngineError {
    #[error("Cannot find transaction: {0}")]
    TransactionNotFound(TransactionId),
    #[error("Client account is frozen: {0}")]
    FrozenAccount(ClientId),
    #[error("Precision is invalid for transaction: {0}")]
    InvalidPrecision(TransactionId),
    #[error("Negative amount detected for transaction: {0}")]
    NegativeAmount(TransactionId),
    #[error("Not enough funds to process transaction: {0}")]
    NotEnoughMany(TransactionId),
    #[error("Deposit or withdraw need to has amount")]
    MissingAmount(),
    #[error("Csv error: {0}")]
    Csv(String),
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Input file not provided")]
    InputNotProvided(),
}

impl From<std::io::Error> for EngineError {
    fn from(error: std::io::Error) -> Self {
        EngineError::FileNotFound(error.to_string())
    }
}

impl From<csv::Error> for EngineError {
    fn from(error: csv::Error) -> Self {
        EngineError::Csv(error.to_string())
    }
}
