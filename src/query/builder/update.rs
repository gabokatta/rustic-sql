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

#[cfg(test)]
mod tests {
    use crate::query::structs::expression::ExpressionNode::Empty;
    use crate::query::structs::expression::{ExpressionNode, ExpressionOperator};
    use crate::query::structs::operation::Operation::Update;
    use crate::query::structs::query::Query;
    use crate::query::structs::token::TokenKind::{Identifier, Number};
    use crate::query::structs::token::{Token, TokenKind};
    use crate::query::tokenizer::Tokenizer;

    fn tokenize(sql: &str) -> Vec<Token> {
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(sql).unwrap()
    }

    fn to_token(value: &str, kind: TokenKind) -> Token {
        Token {
            value: value.to_string(),
            kind,
        }
    }

    #[test]
    fn test_update_simple() {
        let sql = "UPDATE ordenes SET id = 5";
        let tokens = tokenize(sql);
        let query = Query::from(tokens).unwrap();

        assert_eq!(query.operation, Update);
        assert_eq!(query.table, "ordenes");
        assert_eq!(
            query.updates,
            vec![ExpressionNode::Statement {
                operator: ExpressionOperator::Equals,
                left: Box::new(ExpressionNode::Leaf(to_token("id", Identifier))),
                right: Box::new(ExpressionNode::Leaf(to_token("5", Number))),
            }]
        );
        assert_eq!(query.conditions, Empty);
    }

    #[test]
    fn test_update_with_conditions() {
        let sql = "UPDATE ordenes SET cantidad = 5 WHERE id = 1";
        let tokens = tokenize(sql);
        let query = Query::from(tokens).unwrap();

        assert_eq!(query.operation, Update);
        assert_eq!(query.table, "ordenes");
        assert_eq!(
            query.updates,
            vec![ExpressionNode::Statement {
                operator: ExpressionOperator::Equals,
                left: Box::new(ExpressionNode::Leaf(to_token("cantidad", Identifier))),
                right: Box::new(ExpressionNode::Leaf(to_token("5", Number))),
            }]
        );
        assert_ne!(query.conditions, Empty);
    }

    #[test]
    fn test_update_invalid_keyword() {
        let sql = "UPDATE ordenes SET quantity = 5 ORDER BY id";
        let tokens = tokenize(sql);
        let result = Query::from(tokens);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("ORDER BY"));
    }

    #[test]
    fn test_update_missing_set() {
        let sql = "UPDATE ordenes quantity = 5";
        let tokens = tokenize(sql);
        let result = Query::from(tokens);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("SET"));
    }
}
