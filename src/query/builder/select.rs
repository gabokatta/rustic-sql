use crate::query::builder::{validate_keywords, Builder};
use crate::query::errors::InvalidSQL;
use crate::query::Operation::Select;
use crate::query::{Ordering, Query, Statement, Token};
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

    fn parse_table(&self) -> String {
        "".to_string()
    }

    fn parse_fields(&self) -> Vec<Token> {
        vec![]
    }

    fn parse_ordering(&self) -> Ordering {
        Ordering::default()
    }

    fn parse_statements(&self) -> Vec<Statement> {
        vec![]
    }
}

impl Builder for SelectBuilder {
    fn build(&mut self) -> Result<Query, InvalidSQL> {
        let mut query = Query::default();
        self.validate_keywords()?;

        query.operation = Select;
        query.table = self.parse_table();
        query.fields = self.parse_fields();
        query.expressions = self.parse_statements();
        query.ordering = self.parse_ordering();

        Ok(query)
    }

    fn validate_keywords(&self) -> Result<(), InvalidSQL> {
        validate_keywords(ALLOWED_KEYWORDS, &self.tokens, Select)
    }
}
