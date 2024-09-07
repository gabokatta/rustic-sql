use crate::errored;
use crate::query::builder::expression::{ExpressionBuilder, ExpressionNode};
use crate::query::builder::{validate_keywords, Builder};
use crate::query::errors::InvalidSQL;
use crate::query::errors::InvalidSQL::Syntax;
use crate::query::Operation::Update;
use crate::query::TokenKind::Keyword;
use crate::query::{Query, Token};
use std::collections::VecDeque;

const ALLOWED_KEYWORDS: &[&str] = &["SET", "WHERE", "AND", "OR"];

pub struct UpdateBuilder {
    tokens: VecDeque<Token>,
}

impl UpdateBuilder {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self { tokens }
    }

    fn parse_updates(&mut self) -> Result<Vec<ExpressionNode>, InvalidSQL> {
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
    fn build(&mut self) -> Result<Query, InvalidSQL> {
        let mut query = Query::default();
        self.validate_keywords()?;
        query.operation = Update;
        query.table = self.parse_table(Update)?;
        query.updates = self.parse_updates()?;
        query.conditions = self.parse_where()?;
        Ok(query)
    }

    fn tokens(&mut self) -> &mut VecDeque<Token> {
        &mut self.tokens
    }

    fn validate_keywords(&self) -> Result<(), InvalidSQL> {
        validate_keywords(ALLOWED_KEYWORDS, &self.tokens, Update)
    }
}
