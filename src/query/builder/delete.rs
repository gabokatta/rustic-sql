use crate::query::builder::{unexpected_token_in_stage, validate_keywords, Builder};
use crate::query::errors::InvalidSQL;
use crate::query::Operation::Delete;
use crate::query::{Query, Token};
use std::collections::VecDeque;

const ALLOWED_KEYWORDS: &[&str] = &["FROM", "WHERE", "AND", "OR"];

pub struct DeleteBuilder {
    tokens: VecDeque<Token>,
}

impl DeleteBuilder {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self { tokens }
    }
}

impl Builder for DeleteBuilder {
    fn build(&mut self) -> Result<Query, InvalidSQL> {
        let mut query = Query::default();
        self.validate_keywords()?;

        query.operation = Delete;
        query.table = self.parse_table(Delete)?;
        query.conditions = self.parse_where()?;

        Ok(query)
    }

    fn tokens(&mut self) -> &mut VecDeque<Token> {
        &mut self.tokens
    }

    fn validate_keywords(&self) -> Result<(), InvalidSQL> {
        validate_keywords(ALLOWED_KEYWORDS, &self.tokens, Delete)
    }
}
