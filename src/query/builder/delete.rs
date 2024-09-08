use crate::query::builder::{validate_keywords, Builder};
use crate::query::errors::InvalidSQL;
use crate::query::structs::operation::Operation::Delete;
use crate::query::structs::query::Query;
use crate::query::structs::token::Token;
use crate::query::structs::token::TokenKind::Keyword;
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
        match self.peek_expecting("WHERE", Keyword) {
            Ok(_) => {
                query.conditions = self.parse_where()?;
            }
            Err(_) => self.expect_none()?,
        }
        Ok(query)
    }

    fn tokens(&mut self) -> &mut VecDeque<Token> {
        &mut self.tokens
    }

    fn validate_keywords(&self) -> Result<(), InvalidSQL> {
        validate_keywords(ALLOWED_KEYWORDS, &self.tokens, Delete)
    }
}
