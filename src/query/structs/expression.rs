use crate::errored;
use crate::query::structs::comparator::ExpressionComparator;
use crate::query::structs::expression::ExpressionResult::{Bool, Int, Str};
use crate::query::structs::token::{Token, TokenKind};
use crate::utils::errors::Errored;
use crate::utils::errors::Errored::{Column, Syntax};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

#[derive(Default, PartialEq)]
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

pub enum ExpressionResult {
    Int(i64),
    Str(String),
    Bool(bool),
}

impl ExpressionNode {
    pub fn evaluate(&self, values: &HashMap<String, String>) -> Result<ExpressionResult, Errored> {
        match self {
            ExpressionNode::Empty => Ok(Bool(false)),
            ExpressionNode::Leaf(t) => match t.kind {
                TokenKind::Identifier => ExpressionNode::get_variable_value(values, t),
                TokenKind::String => Ok(Str(t.value.to_string())),
                TokenKind::Number => Ok(Int(t.value.parse::<i64>()?)),
                _ => Ok(Bool(false)),
            },
            ExpressionNode::Statement {
                operator,
                left,
                right,
            } => {
                let l = left.evaluate(values)?;
                let r = right.evaluate(values)?;
                ExpressionNode::get_statement_value(operator, l, r)
            }
        }
    }

    fn get_statement_value(
        operator: &ExpressionOperator,
        left: ExpressionResult,
        right: ExpressionResult,
    ) -> Result<ExpressionResult, Errored> {
        match (left, right) {
            (Int(l), Int(r)) => ExpressionComparator::compare_ints(l, r, operator),
            (Str(l), Str(r)) => ExpressionComparator::compare_str(&l, &r, operator),
            (Bool(l), Bool(r)) => ExpressionComparator::compare_bools(l, r, operator),
            _ => errored!(Syntax, "expression members must match in type."),
        }
    }

    fn get_variable_value(
        values: &HashMap<String, String>,
        t: &Token,
    ) -> Result<ExpressionResult, Errored> {
        let value = values.get(&t.value);
        match value {
            Some(v) => {
                if v.parse::<i64>().is_ok() {
                    Ok(Int(v.parse::<i64>()?))
                } else {
                    Ok(Str(v.to_string()))
                }
            }
            None => errored!(Column, "column {} does not exist", t.value),
        }
    }
}

impl Debug for ExpressionNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionNode::Empty => write!(f, "()"),
            ExpressionNode::Leaf(t) => write!(f, "{}", t.value),
            ExpressionNode::Statement {
                operator,
                left,
                right,
            } => {
                write!(f, "{:?}[{:?},{:?}]", operator, left, right)
            }
        }
    }
}
