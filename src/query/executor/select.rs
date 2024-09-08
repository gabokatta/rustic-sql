use crate::query::executor::Executor;
use crate::query::structs::row::Row;
use crate::utils::errors::Errored;
use crate::utils::files::{extract_header, split_csv};
use std::fs::File;
use std::io::{BufRead, BufReader};

impl Executor {
    pub fn run_select(&mut self, table: File) -> Result<(), Errored> {
        let mut reader = BufReader::new(table);
        let header = extract_header(&mut reader)?;
        println!("{}", header.join(","));
        let mut matched_rows: Vec<Row> = vec![];
        for line in reader.lines() {
            let l = line?;
            let fields = split_csv(&l);
            let mut row = Row::new(&header);
            row.set_new_values(fields)?;
            if row.matches_condition(&self.query)? {
                matched_rows.push(row)
            }
        }
        //todo: implement ordering.
        Ok(())
    }
}
