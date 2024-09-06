use crate::errored;
use crate::query::errors::InvalidSQL;
use crate::query::errors::InvalidSQL::Syntax;
use crate::query::TokenKind::ParenthesisClose;
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
    pub fn new() -> Self {
        Self
    }

    pub fn parse_expressions(
        &self,
        tokens: &mut VecDeque<Token>,
    ) -> Result<ExpressionNode, InvalidSQL> {
        self.parse_statement(tokens)
    }

    fn parse_leaf(&self, tokens: &mut VecDeque<Token>) -> Result<ExpressionNode, InvalidSQL> {
        if let Some(t) = tokens.pop_front() {
            match t.kind {
                TokenKind::String | TokenKind::Number | TokenKind::Identifier => {
                    Ok(ExpressionNode::Leaf(t))
                }
                TokenKind::ParenthesisOpen => {
                    let node = self.parse_expressions(tokens)?;
                    if let Some(next) = tokens.front() {
                        if next.kind != ParenthesisClose {
                            errored!(Syntax, "expected closing parenthesis, got: {:?}", t)
                        }
                    }
                    Ok(node)
                }
                _ => errored!(Syntax, "invalid token during WHERE parsing: {:?}", t),
            }
        } else {
            errored!(Syntax, "missing tokens while parsing conditions.")
        }
    }

    fn parse_statement(&self, tokens: &mut VecDeque<Token>) -> Result<ExpressionNode, InvalidSQL> {
        let mut left = self.parse_leaf(tokens)?;
        while let Some(t) = tokens.front() {
            let operator: ExpressionOperator = match t.kind {
                TokenKind::Operator => match t.value.as_str() {
                    ">" => ExpressionOperator::GreaterThan,
                    "=" => ExpressionOperator::Equals,
                    "<" => ExpressionOperator::LessThan,
                    ">=" => ExpressionOperator::GreaterOrEqual,
                    "<=" => ExpressionOperator::LessOrEqual,
                    "!=" => ExpressionOperator::NotEquals,
                    _ => ExpressionOperator::None,
                },
                TokenKind::Keyword => match t.value.as_str() {
                    "AND" => ExpressionOperator::And,
                    "OR" => ExpressionOperator::Or,
                    "NOT" => ExpressionOperator::Not,
                    _ => ExpressionOperator::None,
                },
                _ => ExpressionOperator::None,
            };
            if operator == ExpressionOperator::None {
                break;
            }
            tokens.pop_front();

            let right = self.parse_leaf(tokens)?;
            left = ExpressionNode::Statement {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            }
        }

        Ok(left)
    }
}
