use crate::query::builder::Builder;
use crate::query::errors::InvalidSQL;
use crate::query::{Query, Token};
use std::collections::VecDeque;

const ALLOWED_KEYWORDS: &[&str] = &["VALUES"];

pub struct InsertBuilder {
    tokens: VecDeque<Token>,
}

impl InsertBuilder {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self { tokens }
    }
}

impl Builder for InsertBuilder {
    fn build(&mut self) -> Result<Query, InvalidSQL> {
        todo!()
    }
}
