pub mod builder;
mod errors;
mod executor;
pub mod tokenizer;

use crate::errors::Errored;
use crate::query::OrderKind::Asc;
use crate::query::StatementKind::Condition;
use crate::query::TokenKind::Unknown;

#[derive(Debug)]
pub struct Query {
    pub operation: Operation,
    pub table: String,
    fields: Vec<Token>,
    expressions: Vec<Statement>,
    ordering: Ordering,
}

#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub kind: TokenKind,
}

#[derive(Debug)]
struct Statement {
    kind: StatementKind,
    operator: Token,
    left: Token,
    right: Token,
}

#[derive(Debug)]
struct Ordering {
    fields: Vec<Token>,
    kind: OrderKind,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Unknown,
    String,
    Number,
    Operator,
    Identifier,
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

#[derive(Debug, PartialEq)]
enum StatementKind {
    Condition,
    Assignment,
}

#[derive(Debug)]
enum OrderKind {
    Asc,
    Desc,
}

impl Ordering {
    pub fn default() -> Self {
        Self {
            fields: vec![],
            kind: Asc,
        }
    }
}

impl Statement {
    pub fn default() -> Self {
        Self {
            kind: Condition,
            operator: Token::default(),
            left: Token::default(),
            right: Token::default(),
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
            fields: vec![],
            expressions: vec![],
            ordering: Ordering::default(),
        }
    }
}

pub fn validate_query_string(query: &str) -> Result<(), Errored> {
    if query.trim().is_empty() {
        return Err(Errored(String::from("query is empty.")));
    }
    Ok(())
}
