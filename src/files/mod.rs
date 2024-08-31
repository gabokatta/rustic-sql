use errors::FileError;
use std::path::Path;
use FileError::{EmptyDirectory, IOError, InvalidDirectory, InvalidPath};

mod errors;

pub fn validate_path(dir: &str) -> Result<(), FileError> {
    let path = Path::new(dir);

    if !path.exists() {
        return Err(InvalidPath(dir.to_string()));
    }
    if !path.is_dir() {
        return Err(InvalidDirectory(dir.to_string()));
    }

    let dir_entries = path.read_dir().map_err(IOError)?;
    if dir_entries.count() == 0 {
        return Err(EmptyDirectory(dir.to_string()));
    }
    Ok(())
}
