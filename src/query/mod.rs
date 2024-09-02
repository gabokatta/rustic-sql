mod builder;
mod errors;
mod executor;
pub mod tokenizer;

use crate::errors::Errored;
use crate::query::TokenKind::Unknown;

#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub kind: TokenKind,
}

impl Token {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            kind: Unknown,
        }
    }
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

#[derive(Debug)]
pub struct Query {}

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
    ]
}

pub fn validate_query_string(query: &str) -> Result<(), Errored> {
    if query.trim().is_empty() {
        return Err(Errored(String::from("query is empty.")));
    }
    Ok(())
}
