mod builder;
mod errors;
mod executor;
pub mod tokenizer;

use crate::errors::Errored;
use crate::query::Ordering::Asc;
use crate::query::StatementKind::Condition;
use crate::query::TokenKind::Unknown;

pub struct Query {
    pub operation: Operation,
    pub table: String,
    fields: Option<Vec<Token>>,
    expressions: Option<Vec<Statement>>,
    ordering: Order,
}

#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub kind: TokenKind,
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

struct Statement {
    kind: StatementKind,
    operator: Token,
    left: Token,
    right: Token,
}

#[derive(Debug)]
struct Order {
    fields: Vec<Token>,
    order: Ordering,
}

#[derive(Debug, PartialEq)]
enum StatementKind {
    Condition,
    Assignment,
}

#[derive(Debug)]
enum Ordering {
    Asc,
    Desc,
}

impl Order {
    pub fn default() -> Self {
        Self {
            fields: vec![],
            order: Asc,
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
            fields: None,
            expressions: None,
            ordering: Order::default(),
        }
    }
}

fn char_at(index: usize, string: &str) -> char {
    string[index..].chars().next().unwrap_or('\0')
}

fn can_be_skipped(c: char) -> bool {
    c.is_whitespace() || ignorable_chars().contains(&c)
}

fn is_identifier_char(c: char) -> bool {
    c == '_' || c.is_alphanumeric() && !can_be_skipped(c)
}

fn is_operator_char(c: char) -> bool {
    valid_operators().contains(&c.to_string())
}

fn valid_operators() -> Vec<String> {
    vec![
        "*".to_string(),
        "=".to_string(),
        "<".to_string(),
        ">".to_string(),
        ">=".to_string(),
        "<=".to_string(),
        "!=".to_string(),
        "<>".to_string(),
    ]
}

fn ignorable_chars() -> Vec<char> {
    vec![' ', '(', ')', ',', ';', '\0', '\n']
}

fn reserved_keywords() -> Vec<String> {
    vec![
        "SELECT".to_string(),
        "UPDATE".to_string(),
        "DELETE".to_string(),
        "INSERT INTO".to_string(),
        "VALUES".to_string(),
        "ORDER BY".to_string(),
        "DESC".to_string(),
        "ASC".to_string(),
        "FROM".to_string(),
        "WHERE".to_string(),
        "AND".to_string(),
        "OR".to_string(),
        "NOT".to_string(),
    ]
}

pub fn validate_query_string(query: &str) -> Result<(), Errored> {
    if query.trim().is_empty() {
        return Err(Errored(String::from("query is empty.")));
    }
    Ok(())
}
