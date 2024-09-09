use crate::errored;
use crate::query::structs::token::TokenKind::{
    Identifier, Keyword, Number, ParenthesisClose, ParenthesisOpen, Unknown,
};
use crate::query::structs::token::{Token, TokenKind};
use crate::query::tokenizer::TokenizerState::*;
use crate::utils::errors::Errored;
use crate::utils::errors::Errored::Syntax;

const VALID_OPERATORS: &[&str] = &["*", "=", "<", ">", "!", ">=", "<=", "!="];

const IGNORABLE_CHARS: &[char] = &[' ', ',', ';', '\0', '\n'];

const RESERVED_KEYWORDS: &[&str] = &[
    "SELECT",
    "UPDATE",
    "DELETE",
    "INSERT INTO",
    "SET",
    "VALUES",
    "ORDER BY",
    "DESC",
    "ASC",
    "FROM",
    "WHERE",
    "AND",
    "OR",
    "NOT",
];

#[derive(Default)]
pub struct Tokenizer {
    i: usize,
    state: TokenizerState,
    parenthesis_count: i8,
}

#[derive(Default)]
enum TokenizerState {
    #[default]
    Begin,
    IdentifierOrKeyword,
    Operator,
    NumberLiteral,
    StringLiteral,
    OpenParenthesis,
    CloseParenthesis,
    Complete,
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {
            i: 0,
            state: Begin,
            parenthesis_count: 0,
        }
    }

    pub fn tokenize(&mut self, sql: &str) -> Result<Vec<Token>, Errored> {
        let mut out = vec![];
        let mut token = Token::default();
        while self.i < sql.len() {
            let c = char_at(self.i, sql);
            match self.state {
                Begin => self.next_state(c)?,
                IdentifierOrKeyword => token = self.tokenize_identifier_or_keyword(sql)?,
                Operator => token = self.tokenize_operator(sql)?,
                NumberLiteral => token = self.tokenize_number(sql)?,
                OpenParenthesis | CloseParenthesis => token = self.tokenize_parenthesis(sql)?,
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
        if self.parenthesis_count != 0 {
            errored!(Syntax, "unclosed parentheses found.")
        }
        Ok(out)
    }

    fn next_state(&mut self, c: char) -> Result<(), Errored> {
        match c {
            c if can_be_skipped(c) => self.i += c.len_utf8(),
            c if c.is_ascii_digit() => self.state = NumberLiteral,
            c if is_identifier_char(c) => self.state = IdentifierOrKeyword,
            '\'' => self.state = StringLiteral,
            '(' => self.state = OpenParenthesis,
            ')' => self.state = CloseParenthesis,
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

    fn tokenize_parenthesis(&mut self, sql: &str) -> Result<Token, Errored> {
        let c = char_at(self.i, sql);
        let mut token = Token::default();
        if c == '(' {
            self.parenthesis_count += 1;
            token.kind = ParenthesisOpen
        } else if c == ')' {
            self.parenthesis_count -= 1;
            token.kind = ParenthesisClose
        } else {
            errored!(Syntax, "unrecognized token {} at char {}", c, self.i)
        }

        self.i += c.len_utf8();
        self.state = Complete;
        token.value = c.to_string();
        Ok(token)
    }

    fn tokenize_identifier_or_keyword(&mut self, sql: &str) -> Result<Token, Errored> {
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

    fn tokenize_number(&mut self, sql: &str) -> Result<Token, Errored> {
        self.tokenize_kind(sql, Number, |c| c.is_ascii_digit())
    }

    fn tokenize_operator(&mut self, sql: &str) -> Result<Token, Errored> {
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

    fn tokenize_quoted(&mut self, sql: &str) -> Result<Token, Errored> {
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
        self.matches_special_tokens(sql, RESERVED_KEYWORDS, is_identifier_char)
    }

    fn matches_operator(&self, sql: &str) -> Option<String> {
        self.matches_special_tokens(sql, VALID_OPERATORS, is_operator_char)
    }

    fn matches_special_tokens<F>(
        &self,
        sql: &str,
        tokens: &[&str],
        matches_kind: F,
    ) -> Option<String>
    where
        F: Fn(char) -> bool,
    {
        for t in tokens {
            let t = t.to_uppercase();
            let end = self.i + t.len();
            if end <= sql.len() {
                let token = &sql[self.i..end];
                let next_char = char_at(end, sql);
                if token.to_uppercase() == t && !matches_kind(next_char) {
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
    ) -> Result<Token, Errored>
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
    c.is_whitespace() || IGNORABLE_CHARS.contains(&c)
}

fn is_identifier_char(c: char) -> bool {
    c == '_' || (c.is_alphanumeric() && !can_be_skipped(c))
}

fn is_operator_char(c: char) -> bool {
    VALID_OPERATORS.contains(&c.to_string().as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::structs::token::TokenKind::{
        Identifier, Keyword, Number, Operator, ParenthesisClose, ParenthesisOpen,
        String as TokenString,
    };

    #[test]
    fn test_tokenize_select() {
        let sql = "SELECT id, producto, id_cliente FROM ordenes WHERE cantidad > 1;";
        let mut tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize(sql).unwrap();

        assert_eq!(tokens.len(), 10);
        assert_eq!(tokens[0].value, "SELECT");
        assert_eq!(tokens[0].kind, Keyword);

        assert_eq!(tokens[1].value, "id");
        assert_eq!(tokens[1].kind, Identifier);

        assert_eq!(tokens[2].value, "producto");
        assert_eq!(tokens[2].kind, Identifier);

        assert_eq!(tokens[3].value, "id_cliente");
        assert_eq!(tokens[3].kind, Identifier);

        assert_eq!(tokens[4].value, "FROM");
        assert_eq!(tokens[4].kind, Keyword);

        assert_eq!(tokens[5].value, "ordenes");
        assert_eq!(tokens[5].kind, Identifier);

        assert_eq!(tokens[6].value, "WHERE");
        assert_eq!(tokens[6].kind, Keyword);

        assert_eq!(tokens[7].value, "cantidad");
        assert_eq!(tokens[7].kind, Identifier);

        assert_eq!(tokens[8].value, ">");
        assert_eq!(tokens[8].kind, Operator);

        assert_eq!(tokens[9].value, "1");
        assert_eq!(tokens[9].kind, Number);
    }

    #[test]
    fn test_tokenize_select_with_parentheses() {
        let sql = "SELECT id FROM t WHERE (a = 1)";
        let mut tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize(sql).unwrap();

        assert_eq!(tokens.len(), 10);
        assert_eq!(tokens[0].value, "SELECT");
        assert_eq!(tokens[0].kind, Keyword);

        assert_eq!(tokens[1].value, "id");
        assert_eq!(tokens[1].kind, Identifier);

        assert_eq!(tokens[2].value, "FROM");
        assert_eq!(tokens[2].kind, Keyword);

        assert_eq!(tokens[3].value, "t");
        assert_eq!(tokens[3].kind, Identifier);

        assert_eq!(tokens[4].value, "WHERE");
        assert_eq!(tokens[4].kind, Keyword);

        assert_eq!(tokens[5].value, "(");
        assert_eq!(tokens[5].kind, ParenthesisOpen);

        assert_eq!(tokens[6].value, "a");
        assert_eq!(tokens[6].kind, Identifier);

        assert_eq!(tokens[7].value, "=");
        assert_eq!(tokens[7].kind, Operator);

        assert_eq!(tokens[8].value, "1");
        assert_eq!(tokens[8].kind, Number);

        assert_eq!(tokens[9].value, ")");
        assert_eq!(tokens[9].kind, ParenthesisClose);
    }

    #[test]
    fn test_tokenize_string_literals() {
        let sql = "SELECT name FROM users WHERE name = 'Alice'";
        let mut tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize(sql).unwrap();
        assert_eq!(tokens.len(), 8);
        assert_eq!(tokens[7].value, "Alice");
        assert_eq!(tokens[7].kind, TokenString);
    }

    #[test]
    fn test_unclosed_parenthesis_error() {
        let sql = "SELECT id FROM ordenes WHERE (producto = 'Laptop'";
        let mut tokenizer = Tokenizer::new();
        let result = tokenizer.tokenize(sql);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("unclosed parentheses found."));
        }
    }

    #[test]
    fn test_unrecognized_char_error() {
        let sql = "SELECT * FROM users WHERE age = @30";
        let mut tokenizer = Tokenizer::new();
        let result = tokenizer.tokenize(sql);

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("could not tokenize char:"));
        }
    }

    #[test]
    fn test_tokenize_with_operators() {
        let sql = "SELECT * FROM users WHERE age >= 30";
        let mut tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize(sql).unwrap();

        assert_eq!(tokens.len(), 8);
        assert_eq!(tokens[1].value, "*");
        assert_eq!(tokens[1].kind, Operator);
        assert_eq!(tokens[6].value, ">=");
        assert_eq!(tokens[6].kind, Operator);
    }
}
