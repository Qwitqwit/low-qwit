use chrono::{DateTime, Datelike};
use core::fmt::Display;

const SEVENTY_YEARS_IN_DAYS: i64 = 25569;

trait ToUnixEpoch {
    fn convert(self) -> i64;
}

pub struct ExcelEpoch(i64);

impl ExcelEpoch {
    pub fn as_unix(&self) -> i64 {
        self.0 - SEVENTY_YEARS_IN_DAYS
    }
}

impl Display for ExcelEpoch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_hours = self.as_unix() * 24;
        let as_minutes = as_hours * 60;
        let as_sec = as_minutes * 60;
        let utc = DateTime::from_timestamp(as_sec, 0).unwrap();
        let day = utc.day();
        let month = utc.month();
        let year = utc.year();
        f.write_str(&format!("{day}/{month}/{year}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_excel_epoch_to_string() {
        assert_eq!(ExcelEpoch(32121).to_string().as_str(), "10/12/1987");
        assert_eq!(ExcelEpoch(32122).to_string().as_str(), "11/12/1987");
    }

    #[test]
    fn from_excel_epoch_to_unix_epoch() {
        assert_eq!(ExcelEpoch(32121).as_unix(), 6552);
    }
}
