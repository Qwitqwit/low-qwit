use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};
/// ```
/// use qwitlib::lines::read_lines;
///
///
/// let filename = "hello";
/// let lines = read_lines(filename);
/// ```
pub fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>, QError>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// ```
/// use std::fs::File;
/// use qwitlib::lines::read_file_lines;
///
///
///
/// let tmpfile: File = tempfile::tempfile().unwrap();
/// let lines = read_file_lines(tmpfile);
///```
pub fn read_file_lines(file: File) -> Result<io::Lines<io::BufReader<File>>, QError> {
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Clone)]
pub struct QError(String);

impl From<io::Error> for QError {
    fn from(value: io::Error) -> Self {
        Self(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom};

    use super::*;

    #[test]
    fn opening_file_that_does_not_exist() {
        let filename = "hello_not_existent";
        let lines = read_lines(filename);
        assert!(lines.is_err(), "file should not be there");
    }

    #[test]
    fn opening_file_that_does_exist() {
        let mut tmpfile: File = tempfile::tempfile().unwrap();

        // Seek to start
        tmpfile.seek(SeekFrom::Start(0)).unwrap();

        let lines = read_file_lines(tmpfile);
        assert!(lines.is_ok(), "file should be there");
    }
}
