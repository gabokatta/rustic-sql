use crate::query::executor::Executor;
use crate::utils::errors::Errored;
use crate::utils::files::read_csv_line;
use std::fs::File;
use std::io::{BufRead, BufReader};

impl Executor {
    pub fn run_select(&mut self, table: File) -> Result<(), Errored> {
        let mut reader = BufReader::new(table);

        let mut header = String::new();
        reader.read_line(&mut header)?;
        let header_fields = read_csv_line(&header);
        println!("{}", header_fields.join(", "));
        for line in reader.lines() {
            let l = line?;
            println!("{:?}", read_csv_line(&l));
        }

        Ok(())
    }
}
