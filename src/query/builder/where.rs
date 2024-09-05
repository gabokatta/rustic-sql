use crate::query::errors::InvalidSQL;
use crate::query::Statement;

pub struct WhereBuilder;

impl WhereBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_conditions(&self) -> Result<Vec<Statement>, InvalidSQL> {
        Ok(vec![])
    }
}
