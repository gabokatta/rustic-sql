use crate::query::builder::{unexpected_token_in_stage, validate_keywords, Builder};
use crate::query::errors::InvalidSQL;
use crate::query::structs::operation::Operation::Insert;
use crate::query::structs::query::Query;
use crate::query::structs::token::TokenKind::{Keyword, ParenthesisClose, ParenthesisOpen};
use crate::query::structs::token::{Token, TokenKind};
use std::collections::VecDeque;

const ALLOWED_KEYWORDS: &[&str] = &["VALUES"];

pub struct InsertBuilder {
    tokens: VecDeque<Token>,
}

impl InsertBuilder {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self { tokens }
    }

    fn parse_insert_values(&mut self) -> Result<Vec<Token>, InvalidSQL> {
        self.pop_expecting("VALUES", Keyword)?;
        self.peek_expecting("(", ParenthesisOpen)?;
        let mut values = vec![];
        while let Some(t) = self.tokens.front() {
            match t.kind {
                TokenKind::String | TokenKind::Number => {
                    if let Some(token) = self.tokens.pop_front() {
                        values.push(token);
                    }
                }
                ParenthesisOpen => {
                    self.tokens.pop_front();
                }
                ParenthesisClose => {
                    self.tokens.pop_front();
                }
                _ => unexpected_token_in_stage("VALUES", t)?,
            }
        }
        Ok(values)
    }
}

impl Builder for InsertBuilder {
    fn build(&mut self) -> Result<Query, InvalidSQL> {
        let mut query = Query::default();
        self.validate_keywords()?;
        query.operation = Insert;
        query.table = self.parse_table(Insert)?;
        self.peek_expecting("(", ParenthesisOpen)?;
        query.columns = self.parse_columns()?;
        query.inserts = self.parse_insert_values()?;
        self.expect_none()?;
        Ok(query)
    }

    fn tokens(&mut self) -> &mut VecDeque<Token> {
        &mut self.tokens
    }

    fn validate_keywords(&self) -> Result<(), InvalidSQL> {
        validate_keywords(ALLOWED_KEYWORDS, &self.tokens, Insert)
    }
}
