use crate::errored;
use crate::query::builder::{validate_keywords, Builder};
use crate::query::errors::InvalidSQL;
use crate::query::errors::InvalidSQL::Syntax;
use crate::query::Operation::Insert;
use crate::query::{Query, Token, TokenKind};
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
        self.expect_keyword("VALUES")?;
        let mut closed_values = false;
        let mut values = vec![];
        while let Some(t) = self.tokens.front() {
            if closed_values {
                errored!(Syntax, "invalid tokens after insert value: {:?}", t)
            }
            match t.kind {
                TokenKind::String | TokenKind::Number => {
                    if let Some(token) = self.tokens.pop_front() {
                        values.push(token);
                    }
                }
                TokenKind::ParenthesisOpen => {
                    self.tokens.pop_front();
                }
                TokenKind::ParenthesisClose => {
                    self.tokens.pop_front();
                    closed_values = true;
                }
                _ => {}
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
        query.columns = self.parse_columns()?;
        query.inserts = self.parse_insert_values()?;

        Ok(query)
    }

    fn tokens(&mut self) -> &mut VecDeque<Token> {
        &mut self.tokens
    }

    fn validate_keywords(&self) -> Result<(), InvalidSQL> {
        validate_keywords(ALLOWED_KEYWORDS, &self.tokens, Insert)
    }
}
