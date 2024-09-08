use crate::errored;
use crate::query::builder::expression::ExpressionBuilder;
use crate::query::builder::{validate_keywords, Builder};
use crate::query::structs::expression::ExpressionNode;
use crate::query::structs::operation::Operation::Update;
use crate::query::structs::query::Query;
use crate::query::structs::token::Token;
use crate::query::structs::token::TokenKind::Keyword;
use crate::utils::errors::Errored;
use crate::utils::errors::Errored::Syntax;
use std::collections::VecDeque;

const ALLOWED_KEYWORDS: &[&str] = &["SET", "WHERE", "AND", "OR"];

pub struct UpdateBuilder {
    tokens: VecDeque<Token>,
}

impl UpdateBuilder {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self { tokens }
    }

    fn parse_updates(&mut self) -> Result<Vec<ExpressionNode>, Errored> {
        self.pop_expecting("SET", Keyword)?;
        let mut updates = vec![];
        while let Some(t) = self.tokens.front() {
            if t.kind != Keyword && t.value != "WHERE" {
                let update = ExpressionBuilder::parse_expressions(&mut self.tokens)?;
                match update {
                    ExpressionNode::Statement { .. } => updates.push(update),
                    _ => errored!(
                        Syntax,
                        "failed to parse update statement, got: {:?}",
                        update
                    ),
                }
            } else {
                break;
            }
        }
        Ok(updates)
    }
}

impl Builder for UpdateBuilder {
    fn build(&mut self) -> Result<Query, Errored> {
        let mut query = Query::default();
        self.validate_keywords()?;
        query.operation = Update;
        query.table = self.parse_table(Update)?;
        query.updates = self.parse_updates()?;
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

    fn validate_keywords(&self) -> Result<(), Errored> {
        validate_keywords(ALLOWED_KEYWORDS, &self.tokens, Update)
    }
}
