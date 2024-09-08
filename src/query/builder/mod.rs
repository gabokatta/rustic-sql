pub mod delete;
pub mod expression;
pub mod insert;
pub mod select;
pub mod update;

use crate::errored;
use crate::query::builder::expression::ExpressionBuilder;
use crate::query::errors::InvalidSQL;
use crate::query::errors::InvalidSQL::Syntax;
use crate::query::structs::expression::ExpressionNode;
use crate::query::structs::operation::Operation;
use crate::query::structs::operation::Operation::{Delete, Insert, Select, Unknown, Update};
use crate::query::structs::query::Query;
use crate::query::structs::token::TokenKind::{
    Identifier, Keyword, Operator, ParenthesisClose, ParenthesisOpen,
};
use crate::query::structs::token::{Token, TokenKind};
use std::collections::VecDeque;

pub trait Builder {
    fn build(&mut self) -> Result<Query, InvalidSQL>;
    fn tokens(&mut self) -> &mut VecDeque<Token>;

    fn parse_table(&mut self, operation: Operation) -> Result<String, InvalidSQL> {
        if let Some(t) = self.tokens().front() {
            match operation {
                Select | Delete => {
                    self.peek_expecting("FROM", Keyword)?;
                    self.tokens().pop_front();
                }
                _ => {}
            }
        }
        let t = self
            .tokens()
            .pop_front()
            .ok_or_else(|| Syntax("could not find table identifier.".to_string()))?;
        if t.kind != Identifier {
            unexpected_token_in_stage("TABLE", &t)?;
        }
        Ok(t.value)
    }

    fn parse_columns(&mut self) -> Result<Vec<Token>, InvalidSQL> {
        let mut fields: Vec<Token> = vec![];
        while let Some(t) = self.tokens().front() {
            match t.kind {
                Identifier => {
                    if let Some(op) = self.tokens().pop_front() {
                        fields.push(op);
                    }
                }
                Keyword if t.value == "FROM" || t.value == "VALUES" => {
                    break;
                }
                ParenthesisClose | Operator if t.value == "*" => {
                    self.tokens().pop_front();
                    break;
                }
                ParenthesisOpen => {
                    self.tokens().pop_front();
                }
                _ => unexpected_token_in_stage("COLUMN", t)?,
            }
        }
        Ok(fields)
    }

    fn parse_where(&mut self) -> Result<ExpressionNode, InvalidSQL> {
        self.pop_expecting("WHERE", Keyword)?;
        ExpressionBuilder::parse_expressions(self.tokens())
    }

    fn expect_none(&mut self) -> Result<(), InvalidSQL> {
        if let Some(t) = self.tokens().front() {
            errored!(Syntax, "expected end of query but got: {:?}", t);
        }
        Ok(())
    }

    fn pop_expecting(&mut self, value: &str, kind: TokenKind) -> Result<Option<Token>, InvalidSQL> {
        self.peek_expecting(value, kind)?;
        Ok(self.tokens().pop_front())
    }

    fn peek_expecting(&mut self, value: &str, kind: TokenKind) -> Result<(), InvalidSQL> {
        let expected = Token {
            value: value.to_string(),
            kind,
        };
        if let Some(t) = self.tokens().front() {
            if t.kind != expected.kind || t.value != expected.value.to_uppercase() {
                errored!(Syntax, "expected {:?} token, got: {:?}", expected, t)
            }
        } else {
            errored!(Syntax, "got None when expecting: {:?}", expected)
        }
        Ok(())
    }

    fn validate_keywords(&self) -> Result<(), InvalidSQL>;
}

pub fn get_kind(token: Option<Token>) -> Operation {
    match token {
        Some(t) => match t.value.as_str() {
            "SELECT" => Select,
            "INSERT INTO" => Insert,
            "UPDATE" => Update,
            "DELETE" => Delete,
            _ => Unknown,
        },
        None => Unknown,
    }
}

fn validate_keywords(
    allowed: &[&str],
    tokens: &VecDeque<Token>,
    operation: Operation,
) -> Result<(), InvalidSQL> {
    let keywords: VecDeque<&Token> = tokens.iter().filter(|t| t.kind == Keyword).collect();
    for word in keywords {
        if !allowed.contains(&&*word.value) {
            errored!(
                Syntax,
                "invalid keyword for {:?} query detected: {}",
                operation,
                word.value
            )
        }
    }
    Ok(())
}

pub fn unexpected_token_in_stage(stage: &str, token: &Token) -> Result<(), InvalidSQL> {
    errored!(
        Syntax,
        "unexpected token while parsing {} fields: {:?}",
        stage,
        token
    )
}
