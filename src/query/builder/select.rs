use crate::errored;
use crate::query::builder::r#where::WhereBuilder;
use crate::query::builder::{unexpected_token, validate_keywords, Builder};
use crate::query::errors::InvalidSQL;
use crate::query::errors::InvalidSQL::Syntax;
use crate::query::Operation::Select;
use crate::query::OrderKind::{Asc, Desc};
use crate::query::TokenKind::{Identifier, Keyword, Operator, Unknown};
use crate::query::{OrderKind, Ordering, Query, Token};
use std::collections::VecDeque;

const ALLOWED_KEYWORDS: &[&str] = &[
    "SELECT", "FROM", "WHERE", "ORDER BY", "ASC", "DESC", "AND", "OR",
];

pub struct SelectBuilder {
    tokens: VecDeque<Token>,
    where_builder: WhereBuilder,
}

impl SelectBuilder {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self {
            tokens,
            where_builder: WhereBuilder::new(),
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
                    unexpected_token("TABLE".to_string(), &t)?
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
                _ => unexpected_token("SELECT".to_string(), t)?,
            }
        }
        Ok(fields)
    }

    fn parse_ordering(&mut self) -> Result<Vec<Ordering>, InvalidSQL> {
        let mut ordering = vec![];
        if let Some(t) = self.tokens.front() {
            if t.kind != Keyword || t.value != "ORDER BY" {
                errored!(Syntax, "missing ORDER BY clause, got: {}", t.value)
            }
            self.tokens.pop_front();
        } else {
            return Ok(ordering);
        }
        while let Some(t) = self.tokens.front() {
            if t.kind != Identifier {
                unexpected_token("ORDER_BY fields".to_string(), t)?
            } else if let Some(i) = self.tokens.pop_front() {
                let mut new_order = Ordering::default();
                new_order.field = i;
                ordering.push(new_order)
            }
            if let Some(next) = self.tokens.front() {
                let mut ordering_kind: Option<OrderKind> = None;
                match next.kind {
                    Keyword if next.value == "DESC" => ordering_kind = Some(Desc),
                    _ => {}
                }
                self.tokens.pop_front();
                if let Some(o) = ordering.last_mut() {
                    o.kind = ordering_kind.unwrap_or(Asc);
                }
            }
        }
        Ok(ordering)
    }
}

impl Builder for SelectBuilder {
    fn build(&mut self) -> Result<Query, InvalidSQL> {
        let mut query = Query::default();
        self.validate_keywords()?;

        query.operation = Select;
        query.fields = self.parse_fields()?;
        query.table = self.parse_table()?;
        query.expressions = self.where_builder.parse_conditions()?;
        query.ordering = self.parse_ordering()?;

        Ok(query)
    }

    fn validate_keywords(&self) -> Result<(), InvalidSQL> {
        validate_keywords(ALLOWED_KEYWORDS, &self.tokens, Select)
    }
}

fn match_ordering(token: &Token) -> Result<OrderKind, InvalidSQL> {
    match token.value.as_str() {
        "ASC" => Ok(Asc),
        "DESC" => Ok(Desc),
        _ => errored!(Syntax, "expected ordering operator, got: {}", token.value),
    }
}
