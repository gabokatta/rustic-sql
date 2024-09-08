use crate::errored;
use crate::utils::errors::Errored;
use crate::utils::errors::Errored::Default;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::{fs, thread};

const TEMP_EXTENSION: &str = "tmp";
const CSV_EXTENSION: &str = "csv";
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

pub fn get_table_path(dir_path: &Path, table_name: &str) -> Result<PathBuf, Errored> {
    let table_path = dir_path.join(table_name).with_extension(CSV_EXTENSION);
    if !table_path.is_file() {
        errored!(
            Default,
            "table {} does not exist in directory: {}",
            table_name,
            dir_path.display()
        );
    }
    Ok(table_path)
}

pub fn get_temp_file(table_name: &str, table_path: &Path) -> Result<(File, PathBuf), Errored> {
    let id = thread::current().id();
    let mut hasher = DefaultHasher::new();
    id.hash(&mut hasher);
    let table_path = table_path
        .with_file_name(format!("{}_{}", table_name, hasher.finish()))
        .with_extension(TEMP_EXTENSION);
    Ok((
        File::options()
            .create_new(true)
            .read(true)
            .write(true)
            .truncate(true)
            .open(&table_path)?,
        table_path,
    ))
}

pub fn delete_temp_file(table_path: &Path, temp_path: &Path) -> Result<(), Errored> {
    if let Some(ex) = temp_path.extension() {
        if ex.to_string_lossy() != TEMP_EXTENSION {
            errored!(Default, "tried to delete non_temporary file.")
        }
    }
    fs::rename(temp_path, table_path)?;
    Ok(())
}

pub fn get_table_file(table_path: &Path) -> Result<File, Errored> {
    Ok(File::open(table_path)?)
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
