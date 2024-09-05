use crate::query::builder::{validate_keywords, Builder};
use crate::query::errors::InvalidSQL;
use crate::query::Operation::Update;
use crate::query::{Query, Token};
use std::collections::VecDeque;

const ALLOWED_KEYWORDS: &[&str] = &["SET", "WHERE", "AND", "OR"];

pub struct UpdateBuilder {
    tokens: VecDeque<Token>,
}

impl UpdateBuilder {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self { tokens }
    }
}

impl Builder for UpdateBuilder {
    fn build(&mut self) -> Result<Query, InvalidSQL> {
        let mut query = Query::default();
        query.operation = Update;
        self.validate_keywords()?;

        Ok(query)
    }

    fn validate_keywords(&self) -> Result<(), InvalidSQL> {
        validate_keywords(ALLOWED_KEYWORDS, &self.tokens, Update)
    }
}
