use crate::errors::EngineResult;
use crate::input::reader::InputReader;
use crate::input::row::TransactionRow;
use crate::model::trade::Transaction;
use csv::ReaderBuilder;
use std::fs::File;

pub struct CsvReader {
    iterator: csv::DeserializeRecordsIntoIter<File, TransactionRow>,
}

impl CsvReader {
    pub fn new(path: &str) -> EngineResult<CsvReader> {
        Self::from_file(File::open(path)?)
    }

    pub fn from_file(file: File) -> EngineResult<CsvReader> {
        let reader = ReaderBuilder::new()
            .has_headers(true)
            .trim(csv::Trim::All)
            .from_reader(file);

        Ok(CsvReader {
            iterator: reader.into_deserialize::<TransactionRow>(),
        })
    }
}

impl InputReader for CsvReader {
    fn next(&mut self) -> Option<EngineResult<Transaction>> {
        self.iterator.next().map(to_dto_model)
    }
}

fn to_dto_model(result: Result<TransactionRow, csv::Error>) -> EngineResult<Transaction> {
    match result {
        Ok(transaction) => transaction.try_into(),
        Err(error) => Err(error.into()),
    }
}
