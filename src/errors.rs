use thiserror::Error;

pub type EngineResult<T> = Result<T, EngineError>;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum EngineError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Input file not provided")]
    InputNotProvided(),
}
