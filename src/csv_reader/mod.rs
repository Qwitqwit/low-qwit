use std::path::PathBuf;

use crate::lines::read_lines;

pub fn read_100_and_collect(
    separator: &str,
    filename: &PathBuf,
) -> Result<Vec<Vec<String>>, String> {
    let lines = read(separator, filename)?;
    let hundred = lines.take(100);
    Ok(hundred.collect())
}

pub fn read<'a>(
    separator: &'a str,
    filename: &'a PathBuf,
) -> Result<impl Iterator<Item = Vec<String>> + 'a + 'a, String> {
    let Ok(res) = read_lines(filename) else {
        return Err("Could not find file".to_string());
    };
    let all_ok = res
        .map_while(Result::ok)
        .map(|line| split_to_values(line, separator));
    Ok(all_ok)
}

fn split_to_values(line: String, separator: &str) -> Vec<String> {
    line.split(separator).map(|l| l.to_owned()).collect()
}
