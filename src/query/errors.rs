use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub enum SQLError {
    InvalidSyntax(String),
    InvalidColumn(String),
    InvalidTable(String),
    Error(Box<dyn Error>),
}

impl Error for SQLError {}
impl Debug for SQLError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for SQLError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SQLError::InvalidSyntax(syntax) => {
                write!(f, "INVALID_SYNTAX: {}", syntax)
            }
            SQLError::InvalidColumn(column) => {
                write!(f, "INVALID_COLUMN: {}", column)
            }
            SQLError::InvalidTable(table) => {
                write!(f, "INVALID_TABLE: {}", table)
            }
            SQLError::Error(err) => {
                write!(f, "ERROR: {}", err)
            }
        }
    }
}
