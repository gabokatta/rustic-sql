use crate::errored;
use crate::query::builder::expression::ExpressionNode::Leaf;
use crate::query::builder::expression::ExpressionOperator::{
    Equals, GreaterOrEqual, GreaterThan, LessOrEqual, LessThan, NotEquals,
};
use crate::query::errors::InvalidSQL;
use crate::query::errors::InvalidSQL::Syntax;
use crate::query::TokenKind::{Keyword, ParenthesisClose};
use crate::query::{Token, TokenKind};
use std::collections::VecDeque;

pub struct ExpressionBuilder;

#[derive(Debug, Default, PartialEq)]
pub enum ExpressionOperator {
    #[default]
    None,
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
    And,
    Or,
    Not,
}

#[derive(Default)]
pub enum ExpressionNode {
    #[default]
    Empty,
    Leaf(Token),
    Statement {
        operator: ExpressionOperator,
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
    },
}

impl ExpressionBuilder {
    pub fn parse_expressions(tokens: &mut VecDeque<Token>) -> Result<ExpressionNode, InvalidSQL> {
        ExpressionBuilder::parse_or(tokens)
    }

    fn parse_or(tokens: &mut VecDeque<Token>) -> Result<ExpressionNode, InvalidSQL> {
        let mut left = ExpressionBuilder::parse_and(tokens)?;
        while let Some(t) = tokens.front() {
            if t.kind != Keyword || t.value != "OR" {
                break;
            }
            tokens.pop_front();
            let right = ExpressionBuilder::parse_and(tokens)?;
            left = ExpressionNode::Statement {
                operator: ExpressionOperator::Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_and(tokens: &mut VecDeque<Token>) -> Result<ExpressionNode, InvalidSQL> {
        let mut left = ExpressionBuilder::parse_not(tokens)?;
        while let Some(t) = tokens.front() {
            if t.kind != Keyword || t.value != "AND" {
                break;
            }
            tokens.pop_front();
            let right = ExpressionBuilder::parse_not(tokens)?;
            left = ExpressionNode::Statement {
                operator: ExpressionOperator::And,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_not(tokens: &mut VecDeque<Token>) -> Result<ExpressionNode, InvalidSQL> {
        if let Some(t) = tokens.front() {
            if t.kind == Keyword && t.value == "NOT" {
                tokens.pop_front();
                let node = ExpressionBuilder::parse_comparisons(tokens)?;
                return Ok(ExpressionNode::Statement {
                    operator: ExpressionOperator::Not,
                    left: Box::new(node),
                    right: Box::new(ExpressionNode::Empty),
                });
            }
        }
        ExpressionBuilder::parse_comparisons(tokens)
    }

    fn parse_comparisons(tokens: &mut VecDeque<Token>) -> Result<ExpressionNode, InvalidSQL> {
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

    fn parse_leaf(tokens: &mut VecDeque<Token>) -> Result<ExpressionNode, InvalidSQL> {
        let t = tokens
            .front()
            .ok_or_else(|| Syntax("reached end of query while parsing comparisons.".to_string()))?;
        match t.kind {
            TokenKind::ParenthesisOpen => {
                tokens.pop_front();
                let expression = ExpressionBuilder::parse_expressions(tokens)?;
                tokens
                    .front()
                    .filter(|t| t.kind == ParenthesisClose)
                    .ok_or_else(|| {
                        Syntax("unclosed parenthesis while evaluating WHERE.".to_string())
                    })?;
                tokens.pop_front();
                Ok(expression)
            }
            TokenKind::Identifier | TokenKind::Number | TokenKind::String => tokens
                .pop_front()
                .map(Leaf)
                .ok_or_else(|| Syntax("failed to parse leaf node".to_string())),
            _ => errored!(
                Syntax,
                "unrecognized token while parsing comparison: {:?}.",
                t
            ),
        }
    }

    fn parse_simple_operator(
        tokens: &mut VecDeque<Token>,
    ) -> Result<ExpressionOperator, InvalidSQL> {
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
