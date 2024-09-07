use crate::query::errors::InvalidSQL;
use crate::query::Query;

mod delete;
mod insert;
mod select;
mod update;

pub struct Executor;

impl Executor {
    pub fn run(path: &str, query: Query) -> Result<Vec<String>, InvalidSQL> {
        Ok(vec![])
    }
}
