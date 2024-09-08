use crate::errored;
use crate::query::structs::operation::Operation::*;
use crate::query::structs::query::Query;
use crate::utils::errors::Errored;
use crate::utils::errors::Errored::{Syntax, Table};
use crate::utils::files::{get_table_file, read_csv_line};
use std::collections::HashMap;

mod delete;
mod insert;
mod select;
mod update;

pub struct Executor {
    path: String,
    query: Query,
    values: HashMap<String, String>,
}

impl Executor {
    pub fn new(path: &str, query: Query) -> Self {
        Executor {
            path: path.to_string(),
            query,
            values: HashMap::new(),
        }
    }

    pub fn run(path: &str, query: Query) -> Result<(), Errored> {
        let mut executor = Executor::new(path, query);
        let table = get_table_file(&executor.path, &executor.query.table)?;
        match executor.query.operation {
            Select => executor.run_select(table),
            Update => executor.run_update(table),
            Delete => executor.run_delete(table),
            Insert => executor.run_insert(table),
            _ => errored!(Syntax, "unknown operation trying to be executed."),
        }
    }
}
