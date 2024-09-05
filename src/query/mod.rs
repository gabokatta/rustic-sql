pub mod builder;
mod errors;
mod executor;
pub mod tokenizer;

use crate::errors::Errored;
use crate::query::OrderKind::Asc;
use crate::query::StatementKind::Condition;
use crate::query::TokenKind::Unknown;
use std::fmt::{Debug, Display, Formatter};

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

impl Display for Query {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Query Kind: [{:?}]", self.operation)?;
        writeln!(f, "Table: {:?}", self.table)?;
        let fields: Vec<&str> = self.fields.iter().map(|f| f.value.as_str()).collect();
        writeln!(f, "Fields: {:?}", fields)?;
        writeln!(f, "Expressions: {:?}", self.expressions)?;
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

pub fn validate_query_string(query: &str) -> Result<(), Errored> {
    if query.trim().is_empty() {
        return Err(Errored(String::from("query is empty.")));
    }
    Ok(())
}
