use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io;
pub enum FileError {
    InvalidPath(String),
    InvalidDirectory(String),
    EmptyDirectory(String),
    IOError(io::Error),
}

impl Error for FileError {}
impl Debug for FileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for FileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FileError::InvalidDirectory(path) => {
                write!(
                    f,
                    "FileError: Path {} points to an invalid directory.",
                    path
                )
            }
            FileError::EmptyDirectory(path) => {
                write!(f, "FileError: Path {} points to an empty directory.", path)
            }
            FileError::InvalidPath(path) => {
                write!(f, "FileError: Path {} does not exist.", path)
            }
            FileError::IOError(e) => {
                write!(f, "IOError: {}", e)
            }
        }
    }
}
