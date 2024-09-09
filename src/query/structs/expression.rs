use crate::errored;
use crate::query::structs::comparator::ExpressionComparator;
use crate::query::structs::expression::ExpressionResult::{Bool, Int, Str};
use crate::query::structs::token::{Token, TokenKind};
use crate::utils::errors::Errored;
use crate::utils::errors::Errored::{Column, Default, Syntax};
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

#[derive(Debug, PartialEq)]
pub enum ExpressionResult {
    Int(i64),
    Str(String),
    Bool(bool),
}

impl ExpressionNode {
    pub fn evaluate(&self, values: &HashMap<String, String>) -> Result<ExpressionResult, Errored> {
        match self {
            ExpressionNode::Empty => Ok(Bool(true)),
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

    pub fn get_variable_value(
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

    pub fn as_leaf_tuple(&self) -> Result<(&Token, &Token), Errored> {
        match self {
            ExpressionNode::Statement { left, right, .. } => match (&**left, &**right) {
                (ExpressionNode::Leaf(l), ExpressionNode::Leaf(r)) => Ok((l, r)),
                _ => errored!(Default, "both sides of expression must be leaf nodes."),
            },
            _ => errored!(Default, "expected a statement, but got: {:?}", self),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::structs::token::Token;
    use crate::query::structs::token::TokenKind::*;
    use std::collections::HashMap;

    #[test]
    fn test_evaluate_empty_node() {
        let node = ExpressionNode::Empty;
        assert_eq!(node.evaluate(&HashMap::new()).unwrap(), Bool(true));
    }

    #[test]
    fn test_evaluate_leaf_identifier() {
        let mut values = HashMap::new();
        values.insert("id_cliente".to_string(), "123".to_string());
        let node = ExpressionNode::Leaf(Token {
            kind: Identifier,
            value: "id_cliente".to_string(),
        });
        assert_eq!(node.evaluate(&values).unwrap(), Int(123));
    }

    #[test]
    fn test_evaluate_leaf_string() {
        let node = ExpressionNode::Leaf(Token {
            kind: String,
            value: "buenaaaaas".to_string(),
        });
        assert_eq!(
            node.evaluate(&HashMap::new()).unwrap(),
            Str("buenaaaaas".to_string())
        );
    }

    #[test]
    fn test_evaluate_leaf_number() {
        let node = ExpressionNode::Leaf(Token {
            kind: Number,
            value: "360".to_string(),
        });
        assert_eq!(node.evaluate(&HashMap::new()).unwrap(), Int(360));
    }

    #[test]
    fn test_evaluate_invalid_token() {
        let node = ExpressionNode::Leaf(Token {
            kind: Keyword,
            value: "".to_string(),
        });
        assert_eq!(node.evaluate(&HashMap::new()).unwrap(), Bool(false));
    }

    #[test]
    fn test_evaluate_statement_equal_int() {
        let left = ExpressionNode::Leaf(Token {
            kind: Number,
            value: "360".to_string(),
        });
        let right = ExpressionNode::Leaf(Token {
            kind: Number,
            value: "360".to_string(),
        });
        let node = ExpressionNode::Statement {
            operator: ExpressionOperator::Equals,
            left: Box::new(left),
            right: Box::new(right),
        };
        assert_eq!(node.evaluate(&HashMap::new()).unwrap(), Bool(true));
    }

    #[test]
    fn test_evaluate_statement_not_equal_str() {
        let left = ExpressionNode::Leaf(Token {
            kind: String,
            value: "rust".to_string(),
        });
        let right = ExpressionNode::Leaf(Token {
            kind: String,
            value: "gleam".to_string(),
        });
        let node = ExpressionNode::Statement {
            operator: ExpressionOperator::NotEquals,
            left: Box::new(left),
            right: Box::new(right),
        };
        assert_eq!(node.evaluate(&HashMap::new()).unwrap(), Bool(true));
    }

    #[test]
    fn test_get_variable_value_existing() {
        let mut values = HashMap::new();
        values.insert("id_cliente".to_string(), "789".to_string());
        let token = Token {
            kind: Identifier,
            value: "id_cliente".to_string(),
        };
        assert_eq!(
            ExpressionNode::get_variable_value(&values, &token).unwrap(),
            Int(789)
        );
    }

    #[test]
    fn test_get_variable_value_non_existing() {
        let values = HashMap::new();
        let token = Token {
            kind: Identifier,
            value: "id".to_string(),
        };
        assert!(ExpressionNode::get_variable_value(&values, &token).is_err());
    }

    #[test]
    fn test_as_leaf_tuple_valid() {
        let left = ExpressionNode::Leaf(Token {
            kind: Identifier,
            value: "id_cliente".to_string(),
        });
        let right = ExpressionNode::Leaf(Token {
            kind: Identifier,
            value: "360".to_string(),
        });
        let node = ExpressionNode::Statement {
            operator: ExpressionOperator::Equals,
            left: Box::new(left),
            right: Box::new(right),
        };
        let (l, r) = node.as_leaf_tuple().unwrap();
        assert_eq!(l.value, "id_cliente");
        assert_eq!(r.value, "360");
    }

    #[test]
    fn test_as_leaf_tuple_invalid() {
        let left = ExpressionNode::Statement {
            operator: ExpressionOperator::Equals,
            left: Box::new(ExpressionNode::Empty),
            right: Box::new(ExpressionNode::Empty),
        };
        let right = ExpressionNode::Leaf(Token {
            kind: Identifier,
            value: "col1".to_string(),
        });
        let node = ExpressionNode::Statement {
            operator: ExpressionOperator::Equals,
            left: Box::new(left),
            right: Box::new(right),
        };
        assert!(node.as_leaf_tuple().is_err());
    }
}
