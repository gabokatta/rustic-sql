use crate::errored;
use crate::query::structs::operation::Operation::*;
use crate::query::structs::query::Query;
use crate::utils::errors::Errored;
use crate::utils::errors::Errored::Syntax;
use crate::utils::files::get_table_path;
use std::path::{Path, PathBuf};

mod delete;
mod insert;
mod select;
mod update;

pub struct Executor {
    table_path: PathBuf,
    query: Query,
}

impl Executor {
    fn new(table_path: PathBuf, query: Query) -> Self {
        Executor { table_path, query }
    }

    pub fn run(path: &str, query: Query) -> Result<(), Errored> {
        let table_path = get_table_path(Path::new(path), &query.table)?;
        let mut executor = Executor::new(table_path, query);
        match executor.query.operation {
            Select => executor.run_select(),
            Update => executor.run_update(),
            Delete => executor.run_delete(),
            Insert => executor.run_insert(),
            _ => errored!(Syntax, "unknown operation trying to be executed."),
        }
    }
}
