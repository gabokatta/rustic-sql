use crate::errored;
use crate::query::builder::expression::{ExpressionBuilder, ExpressionNode};
use crate::query::builder::{unexpected_token_in_stage, validate_keywords, Builder};
use crate::query::errors::InvalidSQL;
use crate::query::errors::InvalidSQL::Syntax;
use crate::query::Operation::Select;
use crate::query::OrderKind::{Asc, Desc};
use crate::query::TokenKind::{Identifier, Keyword, Operator};
use crate::query::{Ordering, Query, Token};
use std::collections::VecDeque;

const ALLOWED_KEYWORDS: &[&str] = &[
    "SELECT", "FROM", "WHERE", "ORDER BY", "ASC", "DESC", "AND", "OR", "NOT",
];

pub struct SelectBuilder {
    tokens: VecDeque<Token>,
    where_builder: ExpressionBuilder,
}

impl SelectBuilder {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self {
            tokens,
            where_builder: ExpressionBuilder::new(),
        }
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
                    unexpected_token_in_stage("TABLE".to_string(), &t)?
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
                Operator if t.value == "*" => {
                    if let Some(op) = self.tokens.pop_front() {
                        fields.push(op);
                        break;
                    }
                }
                Keyword if t.value == "FROM" => {
                    break;
                }
                _ => unexpected_token_in_stage("SELECT".to_string(), t)?,
            }
        }
        Ok(fields)
    }

    fn parse_ordering(&mut self) -> Result<Vec<Ordering>, InvalidSQL> {
        let mut ordering = vec![];
        if self
            .tokens
            .front()
            .map_or(false, |t| t.kind == Keyword && t.value == "ORDER BY")
        {
            self.tokens.pop_front();
        } else {
            return Ok(ordering);
        }
        while let Some(t) = self.tokens.pop_front() {
            if t.kind != Identifier {
                unexpected_token_in_stage("ORDER_BY".to_string(), &t)?
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

    fn parse_conditions(&mut self) -> Result<ExpressionNode, InvalidSQL> {
        if let Some(t) = self.tokens.front() {
            if t.kind != Keyword || t.value != "WHERE" {
                errored!(Syntax, "missing WHERE clause, got: {}", t.value)
            }
            self.tokens.pop_front();
        }
        self.where_builder.parse_expressions(&mut self.tokens)
    }
}

impl Builder for SelectBuilder {
    fn build(&mut self) -> Result<Query, InvalidSQL> {
        let mut query = Query::default();
        self.validate_keywords()?;

        query.operation = Select;
        query.fields = self.parse_fields()?;
        query.table = self.parse_table()?;
        query.expressions = self.parse_conditions()?;
        query.ordering = self.parse_ordering()?;

        Ok(query)
    }

    fn validate_keywords(&self) -> Result<(), InvalidSQL> {
        validate_keywords(ALLOWED_KEYWORDS, &self.tokens, Select)
    }
}
