use crate::query::builder::{unexpected_token_in_stage, validate_keywords, Builder};
use crate::query::errors::InvalidSQL;
use crate::query::Operation::Select;
use crate::query::OrderKind::{Asc, Desc};
use crate::query::TokenKind::{Identifier, Keyword};
use crate::query::{Ordering, Query, Token};
use std::collections::VecDeque;

const ALLOWED_KEYWORDS: &[&str] = &[
    "SELECT", "FROM", "WHERE", "ORDER BY", "ASC", "DESC", "AND", "OR", "NOT",
];

pub struct SelectBuilder {
    tokens: VecDeque<Token>,
}

impl SelectBuilder {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self { tokens }
    }

    fn parse_ordering(&mut self) -> Result<Vec<Ordering>, InvalidSQL> {
        let mut ordering = vec![];
        while let Some(t) = self.tokens.pop_front() {
            if t.kind != Identifier {
                unexpected_token_in_stage("ORDER_BY", &t)?
            }
            let mut new_order = Ordering::default();
            new_order.field = t;
            if let Some(next) = self.tokens.front() {
                match next.kind {
                    Keyword if next.value == "ASC" || next.value == "DESC" => {
                        new_order.kind = if next.value == "DESC" { Desc } else { Asc };
                        self.tokens.pop_front();
                    }
                    _ => {}
                }
            }
            ordering.push(new_order)
        }
        Ok(ordering)
    }
}

impl Builder for SelectBuilder {
    fn build(&mut self) -> Result<Query, InvalidSQL> {
        let mut query = Query::default();
        self.validate_keywords()?;
        query.operation = Select;
        query.columns = self.parse_columns()?;
        query.table = self.parse_table(Select)?;
        query.conditions = self.parse_where()?;
        if self.pop_expecting("ORDER BY", Keyword).is_ok() {
            query.ordering = self.parse_ordering()?;
        }
        Ok(query)
    }

    fn tokens(&mut self) -> &mut VecDeque<Token> {
        &mut self.tokens
    }

    fn validate_keywords(&self) -> Result<(), InvalidSQL> {
        validate_keywords(ALLOWED_KEYWORDS, &self.tokens, Select)
    }
}
