mod delete;
pub mod expression;
mod insert;
mod select;
mod update;

use crate::errored;
use crate::query::builder::delete::DeleteBuilder;
use crate::query::builder::expression::{ExpressionBuilder, ExpressionNode};
use crate::query::builder::insert::InsertBuilder;
use crate::query::builder::select::SelectBuilder;
use crate::query::builder::update::UpdateBuilder;
use crate::query::errors::InvalidSQL;
use crate::query::errors::InvalidSQL::Syntax;
use crate::query::Operation::{Delete, Insert, Select, Unknown, Update};
use crate::query::TokenKind::{Identifier, Keyword, Operator, ParenthesisClose, ParenthesisOpen};
use crate::query::{Operation, Query, Token};
use std::collections::VecDeque;

pub trait Builder {
    fn build(&mut self) -> Result<Query, InvalidSQL>;
    fn tokens(&mut self) -> &mut VecDeque<Token>;

    fn parse_table(&mut self, operation: Operation) -> Result<String, InvalidSQL> {
        if let Some(t) = self.tokens().front() {
            match operation {
                Select | Delete => {
                    if t.kind != Keyword || t.value != "FROM" {
                        errored!(Syntax, "missing FROM clause, got: {}", t.value)
                    }
                    self.tokens().pop_front();
                }
                Update | Insert => {
                    if t.kind != Identifier {
                        errored!(Syntax, "expected table name, got: {:?}", t)
                    }
                }
                _ => {
                    errored!(Syntax, "unexpected query operation, got: {:?}", t)
                }
            }
        }
        match self.tokens().pop_front() {
            None => errored!(Syntax, "could not find table identifier."),
            Some(t) => {
                if t.kind != Identifier {
                    unexpected_token_in_stage("TABLE".to_string(), &t)?
                }
                Ok(t.value)
            }
        }
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
                Operator if t.value == "*" => {
                    if let Some(op) = self.tokens().pop_front() {
                        fields.push(op);
                        break;
                    }
                }
                Keyword if t.value == "FROM" || t.value == "VALUES" => {
                    break;
                }
                ParenthesisOpen => {
                    self.tokens().pop_front();
                }
                ParenthesisClose => {
                    self.tokens().pop_front();
                    break;
                }
                _ => unexpected_token_in_stage("COLUMN".to_string(), t)?,
            }
        }
        Ok(fields)
    }

    fn parse_where(&mut self) -> Result<ExpressionNode, InvalidSQL> {
        self.expect_keyword("WHERE")?;
        ExpressionBuilder::parse_expressions(self.tokens())
    }

    fn expect_keyword(&mut self, keyword: &str) -> Result<(), InvalidSQL> {
        if let Some(t) = self.tokens().front() {
            if t.kind != Keyword || t.value != keyword.to_uppercase() {
                errored!(
                    Syntax,
                    "missing {} clause, got: {}",
                    keyword.to_uppercase(),
                    t.value
                )
            }
            self.tokens().pop_front();
        }
        Ok(())
    }

    fn validate_keywords(&self) -> Result<(), InvalidSQL>;
}

impl Query {
    pub fn from(tokens: Vec<Token>) -> Result<Self, InvalidSQL> {
        let mut tokens = VecDeque::from(tokens);
        let kind = get_kind(tokens.pop_front());
        match kind {
            Unknown => errored!(Syntax, "query does not start with a valid operation."),
            Select => SelectBuilder::new(tokens).build(),
            Update => UpdateBuilder::new(tokens).build(),
            Delete => DeleteBuilder::new(tokens).build(),
            Insert => InsertBuilder::new(tokens).build(),
        }
    }
}

fn get_kind(token: Option<Token>) -> Operation {
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

fn unexpected_token_in_stage(stage: String, token: &Token) -> Result<(), InvalidSQL> {
    errored!(
        Syntax,
        "unexpected token while parsing {} fields: {} of kind {:?}",
        stage,
        token.value,
        token.kind
    )
}
