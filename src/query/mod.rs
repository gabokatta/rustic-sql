pub mod builder;
mod errors;
pub mod executor;
pub mod tokenizer;

use crate::query::builder::expression::ExpressionNode;
use crate::query::OrderKind::Asc;
use crate::query::TokenKind::Unknown;
use crate::utils::errors::Errored;
use std::fmt::{Debug, Display, Formatter};

pub struct Query {
    pub operation: Operation,
    pub table: String,
    columns: Vec<Token>,
    inserts: Vec<Token>,
    updates: Vec<ExpressionNode>,
    conditions: ExpressionNode,
    ordering: Vec<Ordering>,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub value: String,
    pub kind: TokenKind,
}

struct Ordering {
    field: Token,
    kind: OrderKind,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Unknown,
    String,
    Number,
    Operator,
    Identifier,
    ParenthesisOpen,
    ParenthesisClose,
    Keyword,
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Unknown,
    Select,
    Update,
    Delete,
    Insert,
}

#[derive(Debug)]
enum OrderKind {
    Asc,
    Desc,
}

impl Ordering {
    pub fn default() -> Self {
        Self {
            field: Token::default(),
            kind: Asc,
        }
    }
}

impl Token {
    pub fn default() -> Self {
        Self {
            value: String::new(),
            kind: Unknown,
        }
    }
}

impl Query {
    pub fn default() -> Self {
        Self {
            operation: Operation::Unknown,
            table: "".to_string(),
            columns: vec![],
            inserts: vec![],
            updates: vec![],
            conditions: ExpressionNode::default(),
            ordering: vec![],
        }
    }
}

impl Display for Query {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let fields: Vec<&str> = self.columns.iter().map(|f| f.value.as_str()).collect();
        let inserts: Vec<&str> = self.inserts.iter().map(|f| f.value.as_str()).collect();
        writeln!(f, "Query Kind: [{:?}]", self.operation)?;
        writeln!(f, "Table: {:?}", self.table)?;
        writeln!(f, "Columns: {:?}", fields)?;
        writeln!(f, "Inserts: {:?}", inserts)?;
        writeln!(f, "Updates: {:?}", self.updates)?;
        writeln!(f, "Conditions: {:?}", self.conditions)?;
        writeln!(f, "Ordering: {:?}", self.ordering)
    }
}

impl Debug for Query {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self)
    }
}

impl Debug for Ordering {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{:?})", &self.field.value, &self.kind)
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

pub fn validate_query_string(query: &str) -> Result<(), Errored> {
    if query.trim().is_empty() {
        return Err(Errored(String::from("query is empty.")));
    }
    Ok(())
}
