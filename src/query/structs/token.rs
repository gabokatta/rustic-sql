use crate::query::structs::token::TokenKind::Unknown;

#[derive(Debug, PartialEq)]
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
    ParenthesisOpen,
    ParenthesisClose,
    Keyword,
}

impl Token {
    pub fn default() -> Self {
        Self {
            value: String::new(),
            kind: Unknown,
        }
    }
}
