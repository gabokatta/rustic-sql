use crate::query::errors::InvalidSQL;
use crate::query::errors::InvalidSQL::Syntax;
use crate::query::tokenizer::TokenizerState::*;
use crate::query::TokenKind::{Identifier, Keyword, Number, Unknown};
use crate::query::{
    can_be_skipped, char_at, is_identifier_char, is_operator_char, reserved_keywords,
    valid_operators, Token, TokenKind,
};

pub struct Tokenizer {
    i: usize,
    state: TokenizerState,
}

enum TokenizerState {
    Begin,
    IdentifierOrKeyword,
    Operator,
    NumberLiteral,
    StringLiteral,
    Complete,
}

impl Tokenizer {
    pub fn new() -> Self {
        Self { i: 0, state: Begin }
    }

    pub fn tokenize(&mut self, sql: &str) -> Result<Vec<Token>, InvalidSQL> {
        let mut out = vec![];
        let mut token = Token::new();
        while self.i < sql.len() {
            let c = char_at(self.i, sql);
            match self.state {
                Begin => self.next_state(c)?,
                IdentifierOrKeyword => token = self.tokenize_identifier_or_keyword(sql)?,
                Operator => token = self.tokenize_operator(sql)?,
                NumberLiteral => token = self.tokenize_number(sql)?,
                StringLiteral => {
                    self.i += c.len_utf8();
                    token = self.tokenize_quoted(sql)?;
                }
                Complete => {
                    out.push(token);
                    token = Token::new();
                    self.reset()
                }
            }
        }
        if token.kind != Unknown {
            out.push(token);
        }
        Ok(out)
    }

    fn next_state(&mut self, c: char) -> Result<(), InvalidSQL> {
        match c {
            c if can_be_skipped(c) => self.i += c.len_utf8(),
            c if c.is_ascii_digit() => self.state = NumberLiteral,
            c if is_identifier_char(c) => self.state = IdentifierOrKeyword,
            '\'' => self.state = StringLiteral,
            c if is_operator_char(c) => self.state = Operator,
            _ => {
                return Err(Syntax(format!(
                    "could not tokenize char: {} at index: {}.",
                    c, self.i
                )))
            }
        }
        Ok(())
    }

    fn tokenize_identifier_or_keyword(&mut self, sql: &str) -> Result<Token, InvalidSQL> {
        let start = self.i;
        if let Some(word) = self.matches_keyword(sql) {
            self.i += word.len();
            self.state = Complete;
            return Ok(Token {
                value: word,
                kind: Keyword,
            });
        }

        let mut c = char_at(self.i, sql);
        while self.i < sql.len() && is_identifier_char(c) {
            self.i += c.len_utf8();
            c = char_at(self.i, sql);
        }

        let identifier = &sql[start..self.i];
        self.state = Complete;
        Ok(Token {
            value: String::from(identifier),
            kind: Identifier,
        })
    }

    fn tokenize_operator(&mut self, sql: &str) -> Result<Token, InvalidSQL> {
        if let Some(op) = self.matches_operator(sql) {
            self.i += op.len();
            self.state = Complete;
            Ok(Token {
                value: op,
                kind: TokenKind::Operator,
            })
        } else {
            Err(Syntax(format!(
                "unrecognized operator {} at index: {}",
                char_at(self.i, sql),
                self.i
            )))
        }
    }

    fn tokenize_number(&mut self, sql: &str) -> Result<Token, InvalidSQL> {
        let start = self.i;

        let mut c = char_at(self.i, sql);
        while self.i < sql.len() && c.is_ascii_digit() {
            self.i += c.len_utf8();
            c = char_at(self.i, sql);
        }

        let number = &sql[start..self.i];
        self.state = Complete;
        Ok(Token {
            value: String::from(number),
            kind: Number,
        })
    }

    fn tokenize_quoted(&mut self, sql: &str) -> Result<Token, InvalidSQL> {
        let start = self.i;
        for (index, char) in sql[start..].char_indices() {
            if char == '\'' {
                self.i = start + index + 1;
                let quoted = &sql[start..start + index];
                self.state = Complete;
                return Ok(Token {
                    value: String::from(quoted),
                    kind: TokenKind::String,
                });
            }
        }

        Err(Syntax(format!(
            "unclosed quotation mark after index: {}",
            start
        )))
    }

    fn matches_keyword(&self, sql: &str) -> Option<String> {
        for word in reserved_keywords() {
            let end = self.i + word.len();
            if end <= sql.len() {
                let token = &sql[self.i..end];
                let next_char = char_at(end, sql);
                if token.to_uppercase() == word.as_str() && !is_identifier_char(next_char) {
                    return Some(word);
                }
            }
        }
        None
    }

    fn matches_operator(&self, sql: &str) -> Option<String> {
        for op in valid_operators() {
            let end = self.i + op.len();
            if end <= sql.len() {
                let token = &sql[self.i..end];
                if token == op.as_str() && !is_operator_char(char_at(end, sql)) {
                    return Some(op);
                }
            }
        }
        None
    }

    fn reset(&mut self) {
        self.state = Begin
    }
}
