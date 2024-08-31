use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// Generic Error for the RusticSQL Application.
pub struct Errored(Box<dyn Error>);

impl Error for Errored {}

impl Debug for Errored {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Errored {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ERROR: {}", self.0)
    }
}
