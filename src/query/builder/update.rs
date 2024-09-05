use crate::query::builder::Builder;
use crate::query::errors::InvalidSQL;
use crate::query::{Query, Token};
use std::collections::VecDeque;

const ALLOWED_KEYWORDS: &[&str] = &["SET", "WHERE", "AND", "OR"];

pub struct UpdateBuilder {
    tokens: VecDeque<Token>,
}

impl UpdateBuilder {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self { tokens }
    }
}

impl Builder for UpdateBuilder {
    fn build(&mut self) -> Result<Query, InvalidSQL> {
        todo!()
    }
}
