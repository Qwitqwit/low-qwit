use crate::epoch_conversion::ExcelEpoch;
use calamine::{Data, Range};

pub mod operators;

pub fn write_range(
    range: &Range<Data>,
    mut operator: impl CsvRowOperator,
    sep: String,
) -> Result<(), CsvError> {
    let all_rows = range.rows().map(CsvRow::iterator);
    operator.operate(sep, all_rows)
}

#[derive(Debug)]
pub struct CsvError(String);

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
pub struct CsvValue(Result<String, String>);

#[allow(clippy::cast_possible_truncation)]
impl From<Data> for CsvValue {
    fn from(value: Data) -> Self {
        match value {
            // we do nothing on empty
            Data::Empty => CsvValue(Err("Empty Value".to_owned())),
            // we write for those types
            Data::String(ref s) | Data::DateTimeIso(ref s) | Data::DurationIso(ref s) => {
                // we replace ; with nothing
                let escaped = s.replace(';', "");
                CsvValue(Ok(escaped))
            }

            Data::DateTime(ref f) => {
                let as_int = f.as_f64().round() as i64;
                let epoch = ExcelEpoch(as_int);
                let as_string = epoch.to_string();
                CsvValue(Ok(as_string))
            }

            Data::Float(ref f) => CsvValue(Ok(f.to_string())),
            // we also just write for those
            Data::Int(ref i) => CsvValue(Ok(i.to_string())),
            Data::Bool(ref b) => CsvValue(Ok(b.to_string())),
            Data::Error(ref e) => CsvValue(Err(format!(
                "error in sheet, fix or remove cell error: {e}"
            ))),
        }
    }
}
