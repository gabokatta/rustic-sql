use crate::errored;
use crate::query::errors::InvalidSQL;
use crate::query::errors::InvalidSQL::Syntax;
use crate::query::tokenizer::TokenizerState::*;
use crate::query::TokenKind::{Identifier, Keyword, Number, Unknown};
use crate::query::{Token, TokenKind};

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
        let mut token = Token::default();
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
                    token = Token::default();
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
            _ => errored!(
                Syntax,
                "could not tokenize char: {} at index: {}.",
                c,
                self.i
            ),
        }
        Ok(())
    }

    fn tokenize_identifier_or_keyword(&mut self, sql: &str) -> Result<Token, InvalidSQL> {
        if let Some(word) = self.matches_keyword(sql) {
            self.i += word.len();
            self.state = Complete;
            return Ok(Token {
                value: word,
                kind: Keyword,
            });
        }
        self.tokenize_kind(sql, Identifier, is_identifier_char)
    }

    fn tokenize_number(&mut self, sql: &str) -> Result<Token, InvalidSQL> {
        self.tokenize_kind(sql, Number, |c| c.is_ascii_digit())
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
            errored!(
                Syntax,
                "unrecognized operator {} at index: {}",
                char_at(self.i, sql),
                self.i
            );
        }
    }

    fn tokenize_quoted(&mut self, sql: &str) -> Result<Token, InvalidSQL> {
        let start = self.i;
        for (index, char) in sql[start..].char_indices() {
            if char == '\'' {
                let end = start + index;
                self.i = end + 1;
                let quoted = &sql[start..end];
                self.state = Complete;
                return Ok(Token {
                    value: String::from(quoted),
                    kind: TokenKind::String,
                });
            }
        }
        errored!(Syntax, "unclosed quotation mark after index: {start}");
    }

    fn matches_keyword(&self, sql: &str) -> Option<String> {
        self.matches_special_tokens(sql, &reserved_keywords(), is_identifier_char)
    }

    fn matches_operator(&self, sql: &str) -> Option<String> {
        self.matches_special_tokens(sql, &valid_operators(), is_operator_char)
    }

    fn matches_special_tokens<F>(
        &self,
        sql: &str,
        tokens: &[String],
        matches_kind: F,
    ) -> Option<String>
    where
        F: Fn(char) -> bool,
    {
        for t in tokens {
            let end = self.i + t.len();
            if end <= sql.len() {
                let token = &sql[self.i..end];
                let next_char = char_at(end, sql);
                if token.to_uppercase() == t.to_uppercase() && !matches_kind(next_char) {
                    return Some(token.to_uppercase());
                }
            }
        }
        None
    }

    fn tokenize_kind<F>(
        &mut self,
        sql: &str,
        output_kind: TokenKind,
        matches_kind: F,
    ) -> Result<Token, InvalidSQL>
    where
        F: Fn(char) -> bool,
    {
        let start = self.i;
        let mut c = char_at(self.i, sql);
        while self.i < sql.len() && matches_kind(c) {
            self.i += c.len_utf8();
            c = char_at(self.i, sql);
        }
        let token = &sql[start..self.i];
        self.state = Complete;
        Ok(Token {
            value: String::from(token),
            kind: output_kind,
        })
    }

    fn reset(&mut self) {
        self.state = Begin
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
