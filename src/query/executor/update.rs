use crate::query::executor::Executor;
use crate::utils::errors::Errored;
use crate::utils::files::extract_header;
use std::fs::File;
use std::io::{BufReader, BufWriter};

impl Executor {
    pub fn run_update(&self, table: File) -> Result<(), Errored> {
        let mut writer = BufWriter::new(&table);
        let mut reader = BufReader::new(&table);
        let header = extract_header(&mut reader)?;
        Ok(())
    }
}
