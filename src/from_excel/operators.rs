use std::{
    fs::File,
    io::{BufWriter, Write},
};

use super::{CsvError, CsvRowOperator, CsvValue};

pub struct FileWritingOperator {
    pub writer: BufWriter<File>,
}

impl CsvRowOperator for FileWritingOperator {
    fn operate(
        &mut self,
        separator: String,
        rows: impl Iterator<Item = impl Iterator<Item = CsvValue>>,
    ) -> Result<(), CsvError> {
        rows.for_each(|r| {
            let values: Vec<String> = r.filter_map(|v| v.0.ok()).collect();

            if values.is_empty(){
                return
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
