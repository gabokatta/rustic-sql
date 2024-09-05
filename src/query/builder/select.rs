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

    fn process_table(&self) -> String {
        "".to_string()
    }

    fn process_fields(&self) -> Vec<Token> {
        vec![]
    }

    fn process_ordering(&self) -> Ordering {
        Ordering::default()
    }

    fn process_expressions(&self) -> Vec<Statement> {
        vec![]
    }
}

impl Builder for SelectBuilder {
    fn build(&mut self) -> Result<Query, InvalidSQL> {
        let mut query = Query::default();
        self.validate_keywords()?;

        query.operation = Select;
        query.table = self.process_table();
        query.fields = self.process_fields();
        query.expressions = self.process_expressions();
        query.ordering = self.process_ordering();

        Ok(query)
    }

    fn validate_keywords(&self) -> Result<(), InvalidSQL> {
        validate_keywords(ALLOWED_KEYWORDS, &self.tokens, Select)
    }
}
