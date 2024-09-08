use crate::query::structs::ordering::OrderKind::Asc;
use crate::query::structs::token::Token;
use std::fmt::{Debug, Formatter};

pub struct Ordering {
    pub(crate) field: Token,
    pub(crate) kind: OrderKind,
}

#[derive(Debug)]
pub enum OrderKind {
    Asc,
    Desc,
}

impl Ordering {
    pub fn default() -> Self {
        Self {
            field: Token::default(),
            kind: Asc,
        }
    }
}

impl Debug for Ordering {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{:?})", &self.field.value, &self.kind)
    }
}
