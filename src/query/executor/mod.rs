use crate::errored;
use crate::query::structs::operation::Operation::*;
use crate::query::structs::query::Query;
use crate::utils::errors::Errored;
use crate::utils::errors::Errored::Syntax;
use crate::utils::files::get_table_file;

mod delete;
mod insert;
mod select;
mod update;

pub struct Executor {
    path: String,
    query: Query,
}

impl Executor {
    pub fn new(path: &str, query: Query) -> Self {
        Executor {
            path: path.to_string(),
            query,
        }
    }

    pub fn run(path: &str, query: Query) -> Result<(), Errored> {
        let executor = Executor::new(path, query);
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
