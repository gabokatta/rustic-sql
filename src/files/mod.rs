use crate::errored;
use crate::errors::Errored;
use std::path::Path;

pub fn validate_path(dir: &str) -> Result<(), Errored> {
    let path = Path::new(dir);

    if !path.exists() {
        errored!(Errored, "path '{dir}' does not exist");
    } else if !path.is_dir() {
        errored!(Errored, "path '{dir}' is not a valid directory");
    } else if path.read_dir()?.next().is_none() {
        errored!(Errored, "path '{dir}' is an empty directory");
    }

    Ok(())
}
