use crate::query::executor::Executor;
use crate::utils::errors::Errored;
use std::fs::File;

impl Executor {
    pub fn run_delete(&self, table: File) -> Result<(), Errored> {
        todo!()
    }
}
