use crate::query::builder::{unexpected_token_in_stage, validate_keywords, Builder};
use crate::query::errors::InvalidSQL;
use crate::query::structs::operation::Operation::Select;
use crate::query::structs::ordering::OrderKind::{Asc, Desc};
use crate::query::structs::ordering::Ordering;
use crate::query::structs::query::Query;
use crate::query::structs::token::Token;
use crate::query::structs::token::TokenKind::{Identifier, Keyword};
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
        if self.peek_expecting("WHERE", Keyword).is_ok() {
            query.conditions = self.parse_where()?;
        }
        match self.peek_expecting("ORDER BY", Keyword) {
            Ok(_) => {
                self.tokens.pop_front();
                query.ordering = self.parse_ordering()?;
            }
            Err(_) => self.expect_none()?,
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
