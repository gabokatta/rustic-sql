use crate::query::errors::InvalidSQL;
use crate::query::structs::operation::Operation::*;
use crate::query::structs::query::Query;

mod delete;
mod insert;
mod select;
mod update;

pub struct Executor;

impl Executor {
    pub fn run(path: &str, query: Query) -> Result<Vec<String>, InvalidSQL> {
        match query.operation {
            Select => Executor::run_select(),
            Update => Executor::run_update(),
            Delete => Executor::run_delete(),
            Insert => Executor::run_insert(),
            _ => Ok(vec![]),
        }
    }
}
