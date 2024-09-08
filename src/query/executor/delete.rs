use crate::query::executor::Executor;
use crate::query::structs::row::Row;
use crate::utils::errors::Errored;
use crate::utils::files::{
    delete_temp_file, extract_header, get_table_file, get_temp_file, split_csv,
};
use std::io::{BufRead, BufReader, BufWriter, Write};

impl Executor {
    pub fn run_delete(&self) -> Result<(), Errored> {
        let table = get_table_file(&self.table_path)?;
        let (temp_table, temp_path) = get_temp_file(&self.query.table, &self.table_path)?;
        let mut reader = BufReader::new(&table);
        let mut writer = BufWriter::new(temp_table);
        let header = extract_header(&mut reader)?;
        writeln!(writer, "{}", header.join(","))?;
        for line in reader.lines() {
            let l = line?;
            let fields = split_csv(&l);
            let mut row = Row::new(&header);
            row.set_new_values(fields)?;
            if row.matches_condition(&self.query)? {
                continue;
            } else {
                writeln!(writer, "{}", l)?
            }
        }
        delete_temp_file(&self.table_path, &temp_path)?;
        Ok(())
    }
}
