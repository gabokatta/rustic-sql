use crate::errored;
use crate::query::builder::{validate_keywords, Builder};
use crate::query::errors::InvalidSQL;
use crate::query::errors::InvalidSQL::Syntax;
use crate::query::Operation::Select;
use crate::query::TokenKind::{Identifier, Keyword, Operator};
use crate::query::{Ordering, Query, Statement, Token, TokenKind};
use std::collections::VecDeque;

const ALLOWED_KEYWORDS: &[&str] = &[
    "SELECT", "FROM", "WHERE", "ORDER_BY", "ASC", "DESC", "AND", "OR",
];

pub struct SelectBuilder {
    tokens: VecDeque<Token>,
}

impl SelectBuilder {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self { tokens }
    }

    fn parse_table(&mut self) -> Result<String, InvalidSQL> {
        if let Some(t) = self.tokens.front() {
            if t.kind != Keyword || t.value != "FROM" {
                errored!(Syntax, "missing FROM clause, got: {}", t.value)
            }
            self.tokens.pop_front();
        }
        match self.tokens.pop_front() {
            None => errored!(Syntax, "could not find table identifier."),
            Some(t) => {
                if t.kind != Identifier {
                    errored!(
                        Syntax,
                        "expected table identifier, found {} of kind {:?}",
                        t.value,
                        t.kind
                    )
                }
                Ok(t.value)
            }
        }
    }

    fn parse_fields(&mut self) -> Result<Vec<Token>, InvalidSQL> {
        let mut fields: Vec<Token> = vec![];
        while let Some(t) = self.tokens.front() {
            match t.kind {
                Identifier => {
                    if let Some(op) = self.tokens.pop_front() {
                        fields.push(op);
                    }
                }
                Operator => {
                    if t.value != "*" {
                        errored!(Syntax, "invalid operating in SELECT fields: {}", t.value)
                    }
                    if let Some(op) = self.tokens.pop_front() {
                        fields.push(op);
                        break;
                    }
                }
                Keyword if t.value == "FROM" => {
                    break;
                }
                _ => errored!(
                    Syntax,
                    "unexpected token while parsing SELECT fields: {} of kind {:?}",
                    t.value,
                    t.kind
                ),
            }
        }
        Ok(fields)
    }

    fn parse_ordering(&self) -> Ordering {
        Ordering::default()
    }

    fn parse_conditions(&self) -> Vec<Statement> {
        vec![]
    }
}

impl Builder for SelectBuilder {
    fn build(&mut self) -> Result<Query, InvalidSQL> {
        let mut query = Query::default();
        self.validate_keywords()?;

        query.operation = Select;
        query.fields = self.parse_fields()?;
        query.table = self.parse_table()?;
        query.expressions = self.parse_conditions();
        query.ordering = self.parse_ordering();

        Ok(query)
    }

    fn validate_keywords(&self) -> Result<(), InvalidSQL> {
        validate_keywords(ALLOWED_KEYWORDS, &self.tokens, Select)
    }
}
