use crate::errors::Errored;
use std::path::Path;

pub fn validate_path(dir: &str) -> Result<(), Errored> {
    let path = Path::new(dir);

    if !path.exists() {
        return Err(Errored(format!("path {} does not exist", dir)));
    }
    if !path.is_dir() {
        return Err(Errored(format!("path {} is not a valid directory", dir)));
    }

    let dir_entries = path
        .read_dir()
        .map_err(|e| Errored(format!("failure when reading directory {}: {}", dir, e)))?;
    if dir_entries.count() == 0 {
        return Err(Errored(format!("path {} is an empty directory", dir)));
    }
    Ok(())
}
