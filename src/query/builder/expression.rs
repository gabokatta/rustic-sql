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
