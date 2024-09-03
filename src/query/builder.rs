use crate::query::errors::InvalidSQL;
use crate::query::{Query, Token};

pub struct Builder {
    pub tokens: Vec<Token>,
}

impl Builder {
    pub fn default() -> Self {
        Self { tokens: vec![] }
    }

    pub fn build(tokens: Vec<Token>) -> Result<Query, InvalidSQL> {
        Ok(Query::default())
    }
}
