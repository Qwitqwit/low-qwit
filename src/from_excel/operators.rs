use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::core::CsvError;
use crate::core::CsvRowOperator;
use crate::core::CsvValue;

pub struct FileWritingOperator {
    pub writer: BufWriter<File>,
}

impl CsvRowOperator for FileWritingOperator {
    fn operate(
        &mut self,
        separator: String,
        line_length: i32,
        rows: impl Iterator<Item = impl Iterator<Item = CsvValue>>,
    ) -> Result<(), CsvError> {
        rows.for_each(|r| {
            let mut values: Vec<String> = r.filter_map(|v| v.0.ok()).collect();

            ((values.len() as i32)..line_length)
                .for_each(|_| values.push(CsvValue::empty().0.unwrap()));

            if values.is_empty() {
                return;
            }
            let len = values.len() - 1;
            values.iter().enumerate().for_each(|(n, v)| {
                self.write(v);
                if n != len {
                    self.sep(&separator);
                }
            });

            self.end_line();
        });

        Ok(())
    }
}

pub fn count_columns(
    lines_to_consider: i32,
    rows: impl Iterator<Item = impl Iterator<Item = CsvValue>>,
) -> Result<i32, CsvError> {
    let mut lenghts: Vec<i32> = vec![];
    rows.take(lines_to_consider as usize).for_each(|r| {
        let values: Vec<String> = r.filter_map(|v| v.0.ok()).collect();
        lenghts.push(values.len() as i32)
    });
    let max = lenghts.iter().cloned().fold(i32::MAX, i32::max);
    Ok(max)
}

impl FileWritingOperator {
    fn write(&mut self, value: &str) {
        let _ = write!(&mut self.writer, "{value}").map_err(|err| CsvError(err.to_string()));
    }
    fn end_line(&mut self) {
        write!(self.writer, "\r\n").unwrap();
    }
    fn sep(&mut self, sep: &str) {
        write!(self.writer, "{sep}").unwrap();
    }
}
