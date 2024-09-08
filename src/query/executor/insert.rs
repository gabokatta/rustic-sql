use crate::query::executor::Executor;
use crate::query::structs::row::Row;
use crate::utils::errors::Errored;
use crate::utils::files::{extract_header, get_table_file};
use std::io::{BufReader, Write};

impl Executor {
    pub fn run_insert(&self) -> Result<(), Errored> {
        let mut table = get_table_file(&self.table_path)?;
        let mut reader = BufReader::new(&table);
        let header = extract_header(&mut reader)?;
        for insert in &self.query.inserts {
            let fields: Vec<String> = insert.iter().map(|t| t.value.to_string()).collect();
            let mut row = Row::new(&header);
            row.set_new_values(fields)?;
            writeln!(table, "{}", row.as_csv_string())?
        }
        Ok(())
    }
}
