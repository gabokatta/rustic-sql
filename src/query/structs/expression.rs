use crate::query::structs::token::Token;
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
