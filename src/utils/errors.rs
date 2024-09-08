use crate::utils::errors::Errored::*;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io;

#[macro_export]
macro_rules! errored {
    ($err_type:ident, $msg:expr) => {
        return Err($err_type(format!($msg)))
    };
    ($err_type:ident, $fmt:expr, $($arg:tt)*) => {
        return Err($err_type(format!($fmt, $($arg)*)))
    };
}

/// Generic Error for the RusticSQL Application.
pub enum Errored {
    Syntax(String),
    Column(String),
    Table(String),
    Default(String),
}

impl Error for Errored {}

impl Debug for Errored {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Errored {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Syntax(syntax) => {
                write!(f, "[INVALID_SYNTAX]: {}", syntax)
            }
            Column(column) => {
                write!(f, "[INVALID_COLUMN]: {}", column)
            }
            Table(table) => {
                write!(f, "[INVALID_TABLE]: {}", table)
            }
            Default(error) => {
                write!(f, "[ERROR]: {}", error)
            }
        }
    }
}

impl From<io::Error> for Errored {
    fn from(value: io::Error) -> Self {
        Default(format!("IO Error: {}", value))
    }
}
