mod delete;
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
use crate::query::{Operation, Query, Token};
use std::collections::VecDeque;

pub trait Builder {
    fn build(&mut self) -> Result<Query, InvalidSQL>;
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
