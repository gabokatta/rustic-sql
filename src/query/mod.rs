mod errors;

use crate::errors::Errored;
use crate::query::errors::InvalidSQL;
use crate::query::TokenizerState::Begin;

pub struct Tokenizer {
    index: usize,
    state: TokenizerState,
}

#[derive(Debug)]
pub struct Query {}

#[derive(Debug)]
pub struct Token {
    pub value: String,
    kind: TokenKind,
}

enum TokenizerState {
    Begin,
}

#[derive(Debug)]
enum TokenKind {
    Literal,
    Operator,
    Identifier,
    Keyword,
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {
            index: 0,
            state: Begin,
        }
    }

    pub fn tokenize(&mut self, sql: &String) -> Result<Vec<Token>, InvalidSQL> {
        Ok(vec![])
    }

    fn char_at(&self, index: usize, sql: &str) -> char {
        sql[index..].chars().next().unwrap_or('\0')
    }

    fn reset(&mut self) {
        self.state = Begin
    }
}

fn valid_operators() -> Vec<String> {
    vec![
        "=".to_string(),
        "<".to_string(),
        ">".to_string(),
        ">=".to_string(),
        "<=".to_string(),
        "!=".to_string(),
        "<>".to_string(),
    ]
}

fn skippable_chars() -> Vec<char> {
    vec![' ', '(', ')', ',', ';']
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
