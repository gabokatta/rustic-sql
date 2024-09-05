pub mod builder;
mod errors;
mod executor;
pub mod tokenizer;

use crate::errors::Errored;
use crate::query::OrderKind::Asc;
use crate::query::StatementKind::Condition;
use crate::query::TokenKind::{Identifier, Unknown};

#[derive(Debug)]
pub struct Query {
    pub operation: Operation,
    pub table: String,
    fields: Vec<Token>,
    expressions: Vec<Statement>,
    ordering: Vec<Ordering>,
}

#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub kind: TokenKind,
}

pub enum Expression {
    Condition(Statement),
    Assignment(Statement),
    Group(Vec<Statement>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
}

#[derive(Debug)]
pub struct Statement {
    kind: StatementKind,
    operator: Token,
    left: Token,
    right: Token,
}

#[derive(Debug)]
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

#[derive(Debug, PartialEq)]
pub enum StatementKind {
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
            field: Token::default(),
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
            ordering: vec![],
        }
    }
}

pub fn validate_query_string(query: &str) -> Result<(), Errored> {
    if query.trim().is_empty() {
        return Err(Errored(String::from("query is empty.")));
    }
    Ok(())
}
