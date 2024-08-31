use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub enum InvalidSQL {
    Syntax(String),
    Column(String, String),
    Table(String, String),
}

impl Error for InvalidSQL {}
impl Debug for InvalidSQL {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for InvalidSQL {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidSQL::Syntax(syntax) => {
                write!(f, "[INVALID_SYNTAX]: {}", syntax)
            }
            InvalidSQL::Column(column, details) => {
                write!(
                    f,
                    "[INVALID_COLUMN]: column {} is invalid because {}.",
                    column, details
                )
            }
            InvalidSQL::Table(table, details) => {
                write!(
                    f,
                    "[INVALID_TABLE]: table {} is invalid because {}.",
                    table, details
                )
            }
        }
    }
}
