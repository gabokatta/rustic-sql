use crate::query::builder::{validate_keywords, Builder};
use crate::query::errors::InvalidSQL;
use crate::query::Operation::Insert;
use crate::query::{Query, Token};
use std::collections::VecDeque;

const ALLOWED_KEYWORDS: &[&str] = &["VALUES"];

pub struct InsertBuilder {
    tokens: VecDeque<Token>,
}

impl InsertBuilder {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self { tokens }
    }
}

impl Builder for InsertBuilder {
    fn build(&mut self) -> Result<Query, InvalidSQL> {
        let mut query = Query::default();
        query.operation = Insert;
        self.validate_keywords()?;

        Ok(query)
    }
    fn validate_keywords(&self) -> Result<(), InvalidSQL> {
        validate_keywords(ALLOWED_KEYWORDS, &self.tokens, Insert)
    }
}
