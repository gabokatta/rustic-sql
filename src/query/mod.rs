mod errors;

use crate::errors::Errored;
use crate::query::errors::InvalidSQL;
use crate::query::ParserState::{IdentifierOrKeyword, New, NumberLiteral, Operator, StringLiteral};

pub struct Parser {
    index: usize,
    state: ParserState,
}

pub struct Query {
    pub kind: String,
    pub table: String,
    pub fields: Vec<String>,
    pub conditions: Vec<String>,
}

impl Query {
    pub fn new() -> Self {
        Self {
            kind: String::new(),
            table: String::new(),
            fields: vec![],
            conditions: vec![],
        }
    }
}

impl Parser {
    pub fn new() -> Self {
        Self {
            index: 0,
            state: New,
        }
    }

    pub fn parse(&mut self, sql: String) -> Result<Query, InvalidSQL> {
        let tokens = self.tokenize(sql)?;
        Ok(Query::new())
    }

    pub fn tokenize(&mut self, sql: String) -> Result<Vec<Token>, InvalidSQL> {
        let tokens = vec![];
        while self.index < sql.len() {
            if let Some(char) = self.curr_char(&sql) {
                match &self.state {
                    New => {
                        if char.is_whitespace() {
                            self.index += 1
                        } else if char.is_alphabetic() {
                            self.state = IdentifierOrKeyword
                        } else if char.is_ascii_digit() {
                            self.state = NumberLiteral
                        } else if char == '\'' {
                            self.state = StringLiteral;
                        } else {
                            self.state = Operator
                        }
                    }
                    IdentifierOrKeyword => self.tokenize_identifier_or_keyword(),
                    NumberLiteral => self.tokenize_number_literal(),
                    StringLiteral => self.tokenize_string_literal(),
                    Operator => self.tokenize_operator(),
                }
            } else {
                break;
            }
        }
        Ok(tokens)
    }

    fn tokenize_string_literal(&self) {}

    fn tokenize_number_literal(&self) {}

    fn tokenize_operator(&self) {}

    fn tokenize_identifier_or_keyword(&self) {}

    fn curr_char(&self, sql: &str) -> Option<char> {
        sql[self.index..].chars().next()
    }
}

enum ParserState {
    New,
    IdentifierOrKeyword,
    NumberLiteral,
    StringLiteral,
    Operator,
}

pub struct Token {
    value: String,
    kind: TokenKind,
}

enum TokenKind {
    Keyword,
    Literal,
    Identifier,
    Operator,
}

pub fn validate_query_string(query: &str) -> Result<(), Errored> {
    if query.trim().is_empty() {
        return Err(Errored(String::from("query is empty.")));
    }
    Ok(())
}
