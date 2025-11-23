use crate::errors::EngineResult;
use crate::model::trade::Transaction;

pub trait InputReader {
    fn next(&mut self) -> Option<EngineResult<Transaction>>;
}
