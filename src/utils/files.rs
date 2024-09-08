use crate::errored;
use crate::utils::errors::Errored;
use crate::utils::errors::Errored::{Default, Table};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const CSV_EXTENSION: &str = ".csv";
const CSV_SEPARATOR: &str = ",";

pub fn extract_header(reader: &mut BufReader<&File>) -> Result<Vec<String>, Errored> {
    let mut header = String::new();
    reader.read_line(&mut header)?;
    Ok(split_csv(&header))
}

pub fn split_csv(line: &str) -> Vec<String> {
    line.split(CSV_SEPARATOR)
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>()
}

pub fn validate_path(dir: &str) -> Result<&Path, Errored> {
    let path = Path::new(dir);
    if !path.exists() {
        errored!(Default, "path '{dir}' does not exist");
    } else if !path.is_dir() {
        errored!(Default, "path '{dir}' is not a valid directory");
    } else if path.read_dir()?.next().is_none() {
        errored!(Default, "path '{dir}' is an empty directory");
    }
    Ok(path)
}

pub fn get_table_file(dir_path: &str, table_name: &str) -> Result<File, Errored> {
    let path = Path::new(dir_path);
    let table_path = path.join(format!("{}{}", table_name, CSV_EXTENSION));
    if !table_path.is_file() {
        errored!(
            Default,
            "table {} does not exist in directory: {}",
            table_name,
            dir_path
        );
    }
    match File::open(table_path) {
        Ok(f) => Ok(f),
        Err(err) => errored!(
            Table,
            "could not read table {} file, cause: {}",
            table_name,
            err
        ),
    }
}
