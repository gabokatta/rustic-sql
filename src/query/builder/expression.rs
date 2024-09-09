use crate::errored;
use crate::query::structs::expression::ExpressionNode::{Empty, Leaf};
use crate::query::structs::expression::ExpressionOperator::*;
use crate::query::structs::expression::{ExpressionNode, ExpressionOperator};
use crate::query::structs::token::TokenKind::Keyword;
use crate::query::structs::token::{Token, TokenKind};
use crate::utils::errors::Errored;
use crate::utils::errors::Errored::Syntax;
use std::collections::VecDeque;

pub struct ExpressionBuilder;

impl ExpressionBuilder {
    pub fn parse_expressions(tokens: &mut VecDeque<Token>) -> Result<ExpressionNode, Errored> {
        ExpressionBuilder::parse_or(tokens)
    }

    fn parse_or(tokens: &mut VecDeque<Token>) -> Result<ExpressionNode, Errored> {
        let mut left = ExpressionBuilder::parse_and(tokens)?;
        while let Some(t) = tokens.front() {
            if t.kind != Keyword || t.value != "OR" {
                break;
            }
            tokens.pop_front();
            let right = ExpressionBuilder::parse_and(tokens)?;
            left = ExpressionNode::Statement {
                operator: Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_and(tokens: &mut VecDeque<Token>) -> Result<ExpressionNode, Errored> {
        let mut left = ExpressionBuilder::parse_not(tokens)?;
        while let Some(t) = tokens.front() {
            if t.kind != Keyword || t.value != "AND" {
                break;
            }
            tokens.pop_front();
            let right = ExpressionBuilder::parse_not(tokens)?;
            left = ExpressionNode::Statement {
                operator: And,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_not(tokens: &mut VecDeque<Token>) -> Result<ExpressionNode, Errored> {
        if let Some(t) = tokens.front() {
            if t.kind == Keyword && t.value == "NOT" {
                tokens.pop_front();
                let node = ExpressionBuilder::parse_comparisons(tokens)?;
                return Ok(ExpressionNode::Statement {
                    operator: Not,
                    left: Box::new(node),
                    right: Box::new(Empty),
                });
            }
        }
        ExpressionBuilder::parse_comparisons(tokens)
    }

    fn parse_comparisons(tokens: &mut VecDeque<Token>) -> Result<ExpressionNode, Errored> {
        let left = ExpressionBuilder::parse_leaf(tokens)?;
        let operator = ExpressionBuilder::parse_simple_operator(tokens);
        if operator.is_err() {
            return Ok(left);
        }
        let right = ExpressionBuilder::parse_leaf(tokens)?;
        Ok(ExpressionNode::Statement {
            operator: operator?,
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    fn parse_leaf(tokens: &mut VecDeque<Token>) -> Result<ExpressionNode, Errored> {
        let mut leaf = Empty;
        while let Some(t) = tokens.front() {
            match t.kind {
                TokenKind::Identifier | TokenKind::Number | TokenKind::String => {
                    if let Some(t) = tokens.pop_front() {
                        leaf = Leaf(t);
                        break;
                    }
                }
                TokenKind::ParenthesisOpen => {
                    tokens.pop_front();
                    leaf = ExpressionBuilder::parse_expressions(tokens)?;
                }
                TokenKind::ParenthesisClose if leaf != Empty => {
                    tokens.pop_front();
                    break;
                }
                _ => errored!(Syntax, "unexpected token when parsing leaf: {:?}", t),
            }
        }
        Ok(leaf)
    }

    fn parse_simple_operator(tokens: &mut VecDeque<Token>) -> Result<ExpressionOperator, Errored> {
        let t = tokens
            .front()
            .ok_or_else(|| Syntax("expected operator but was end of query.".to_string()))?;
        let op = match t.value.as_str() {
            "=" => Equals,
            "!=" => NotEquals,
            ">" => GreaterThan,
            ">=" => GreaterOrEqual,
            "<" => LessThan,
            "<=" => LessOrEqual,
            _ => errored!(Syntax, "invalid operator, got: {}", t.value),
        };
        tokens.pop_front();
        Ok(op)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::structs::token::TokenKind::*;
    use crate::query::structs::token::{Token, TokenKind};
    use std::ops::Deref;

    fn create_token(kind: TokenKind, value: &str) -> Token {
        Token {
            kind,
            value: value.to_string(),
        }
    }

    fn operator_should_be(node: &ExpressionNode, op: ExpressionOperator) {
        match node {
            ExpressionNode::Statement { operator, .. } => {
                assert_eq!(*operator, op);
            }
            Empty => {
                if op != None {
                    panic!("Expected a None operator but got {:?}", op)
                }
            }
            _ => panic!("Expected an {:?} expression", op),
        }
    }

    fn leaves_should_have_op(
        node: ExpressionNode,
        l_op: ExpressionOperator,
        r_op: ExpressionOperator,
    ) {
        match node {
            ExpressionNode::Statement { left, right, .. } => {
                operator_should_be(left.deref(), l_op);
                operator_should_be(right.deref(), r_op);
            }
            _ => panic!("Expected an OR expression"),
        }
    }

    #[test]
    fn test_parse_or_expression() {
        let mut tokens = VecDeque::from(vec![
            create_token(Identifier, "x"),
            create_token(Keyword, "="),
            create_token(Number, "1"),
            create_token(Keyword, "OR"),
            create_token(Identifier, "y"),
            create_token(Keyword, "="),
            create_token(Number, "2"),
        ]);

        let result = ExpressionBuilder::parse_expressions(&mut tokens).unwrap();
        operator_should_be(&result, Or);
        leaves_should_have_op(result, Equals, Equals);
    }

    #[test]
    fn test_parse_and_expression() {
        let mut tokens = VecDeque::from(vec![
            create_token(Identifier, "x"),
            create_token(Keyword, "="),
            create_token(Number, "1"),
            create_token(Keyword, "AND"),
            create_token(Identifier, "y"),
            create_token(Keyword, "="),
            create_token(Number, "2"),
        ]);

        let result = ExpressionBuilder::parse_expressions(&mut tokens).unwrap();
        operator_should_be(&result, And);
        leaves_should_have_op(result, Equals, Equals);
    }

    #[test]
    fn test_parse_not_expression() {
        let mut tokens = VecDeque::from(vec![
            create_token(Keyword, "NOT"),
            create_token(ParenthesisOpen, "("),
            create_token(Identifier, "x"),
            create_token(Keyword, "="),
            create_token(Number, "1"),
            create_token(ParenthesisClose, ")"),
        ]);

        let result = ExpressionBuilder::parse_expressions(&mut tokens).unwrap();
        operator_should_be(&result, Not);
        leaves_should_have_op(result, Equals, None)
    }

    #[test]
    fn test_parse_comparison_expression() {
        let mut tokens = VecDeque::from(vec![
            create_token(Identifier, "x"),
            create_token(Keyword, ">"),
            create_token(Number, "10"),
        ]);

        let result = ExpressionBuilder::parse_expressions(&mut tokens).unwrap();
        operator_should_be(&result, GreaterThan)
    }

    #[test]
    fn test_parse_complex_expression() {
        let mut tokens = VecDeque::from(vec![
            create_token(Identifier, "x"),
            create_token(Keyword, "="),
            create_token(Number, "1"),
            create_token(Keyword, "AND"),
            create_token(ParenthesisOpen, "("),
            create_token(Identifier, "y"),
            create_token(Keyword, "="),
            create_token(Number, "2"),
            create_token(Keyword, "OR"),
            create_token(Keyword, "NOT"),
            create_token(Identifier, "z"),
            create_token(Keyword, "="),
            create_token(Number, "3"),
            create_token(ParenthesisClose, ")"),
        ]);

        let result = ExpressionBuilder::parse_expressions(&mut tokens).unwrap();
        operator_should_be(&result, And);
        leaves_should_have_op(result, Equals, Or);
    }

    #[test]
    fn test_parse_invalid_token() {
        let mut tokens = VecDeque::from(vec![
            create_token(Keyword, "INVALID"),
            create_token(Identifier, "x"),
            create_token(Keyword, "="),
            create_token(Number, "1"),
        ]);

        let result = ExpressionBuilder::parse_expressions(&mut tokens);
        assert!(result.is_err());
    }
}
