use crate::errors::Errored;
use std::path::Path;

pub fn validate_path(dir: &str) -> Result<(), Errored> {
    let path = Path::new(dir);

    if !path.exists() {
        return Err(Errored(format!("path '{dir}' does not exist")));
    }
    if !path.is_dir() {
        return Err(Errored(format!("path '{dir}' is not a valid directory")));
    }

    let dir_entries = path
        .read_dir()
        .map_err(|e| Errored(format!("failure when reading directory '{dir}': {e}")))?;
    if dir_entries.count() == 0 {
        return Err(Errored(format!("path '{dir}' is an empty directory")));
    }
    Ok(())
}
