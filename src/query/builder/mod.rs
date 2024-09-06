mod delete;
pub mod expression;
mod insert;
mod select;
mod update;

use crate::errored;
use crate::query::builder::delete::DeleteBuilder;
use crate::query::builder::insert::InsertBuilder;
use crate::query::builder::select::SelectBuilder;
use crate::query::builder::update::UpdateBuilder;
use crate::query::errors::InvalidSQL;
use crate::query::errors::InvalidSQL::Syntax;
use crate::query::Operation::{Delete, Insert, Select, Unknown, Update};
use crate::query::TokenKind::Keyword;
use crate::query::{Operation, Query, Token};
use std::collections::VecDeque;

pub trait Builder {
    fn build(&mut self) -> Result<Query, InvalidSQL>;
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
