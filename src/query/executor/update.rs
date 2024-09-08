use crate::query::executor::Executor;
use crate::utils::errors::Errored;
use crate::utils::files::{extract_header, get_table_file, get_temp_file};
use std::io::BufReader;

impl Executor {
    pub fn run_update(&self) -> Result<(), Errored> {
        let table = get_table_file(&self.table_path)?;
        let temp_table = get_temp_file(&self.table_path)?;
        let mut reader = BufReader::new(&table);
        let header = extract_header(&mut reader)?;
        Ok(())
    }
}
