use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

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
pub struct Errored(pub String);

impl Error for Errored {}

impl Debug for Errored {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Errored {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ERROR]: {}", self.0)
    }
}
