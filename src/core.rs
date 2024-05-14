use std::fmt::Display;

use calamine::Data;

#[derive(Debug)]
pub struct CsvError(pub String);

impl Display for CsvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

pub trait CsvRowOperator {
    fn operate(
        &mut self,
        separator: String,
        rows: impl Iterator<Item = impl Iterator<Item = CsvValue>>,
    ) -> Result<(), CsvError>;
}

pub struct CsvRow;

impl CsvRow {
    pub fn iterator(value: &[Data]) -> impl Iterator<Item = CsvValue> + '_ {
        value.iter().cloned().map(|c| {
            let v: CsvValue = c.into();
            v
        })
    }
}

#[derive(Debug, Clone)]
pub struct CsvValue(pub Result<String, String>);
