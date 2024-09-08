use crate::query::executor::Executor;
use crate::utils::errors::Errored;

impl Executor {
    pub fn run_delete(&self) -> Result<(), Errored> {
        //delete_temp_file(&self.table_path)?;
        Ok(())
    }
}
