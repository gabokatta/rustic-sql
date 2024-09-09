use crate::query::builder::{validate_keywords, Builder};
use crate::query::structs::operation::Operation::Delete;
use crate::query::structs::query::Query;
use crate::query::structs::token::Token;
use crate::query::structs::token::TokenKind::Keyword;
use crate::utils::errors::Errored;
use std::collections::VecDeque;

const ALLOWED_KEYWORDS: &[&str] = &["FROM", "WHERE", "AND", "OR"];

pub struct DeleteBuilder {
    tokens: VecDeque<Token>,
}

impl DeleteBuilder {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self { tokens }
    }
}

impl Builder for DeleteBuilder {
    fn build(&mut self) -> Result<Query, Errored> {
        let mut query = Query::default();
        self.validate_keywords()?;
        query.operation = Delete;
        query.table = self.parse_table(Delete)?;
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
        validate_keywords(ALLOWED_KEYWORDS, &self.tokens, Delete)
    }
}

#[cfg(test)]
mod tests {
    use crate::query::structs::expression::ExpressionNode::Empty;
    use crate::query::structs::operation::Operation::Delete;
    use crate::query::structs::query::Query;
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
    fn test_delete_simple() {
        let sql = "DELETE FROM ordenes";
        let tokens = tokenize(sql);
        let query = Query::from(tokens).unwrap();

        assert_eq!(query.operation, Delete);
        assert_eq!(query.table, "ordenes");
        assert_eq!(query.conditions, Empty);
    }

    #[test]
    fn test_delete_with_conditions() {
        let sql = "DELETE FROM ordenes WHERE id = 1";
        let tokens = tokenize(sql);
        let query = Query::from(tokens).unwrap();

        assert_eq!(query.operation, Delete);
        assert_eq!(query.table, "ordenes");
        assert_ne!(query.conditions, Empty);
    }

    #[test]
    fn test_delete_invalid_keyword() {
        let sql = "DELETE FROM ordenes ORDER BY id";
        let tokens = tokenize(sql);
        let result = Query::from(tokens);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("ORDER BY"));
    }

    #[test]
    fn test_delete_missing_table() {
        let sql = "DELETE WHERE id = 1";
        let tokens = tokenize(sql);
        let result = Query::from(tokens);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("FROM"));
    }
}
