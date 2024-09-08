use crate::errored;
use crate::query::builder::delete::DeleteBuilder;
use crate::query::builder::insert::InsertBuilder;
use crate::query::builder::select::SelectBuilder;
use crate::query::builder::update::UpdateBuilder;
use crate::query::builder::{get_kind, Builder};
use crate::query::errors::InvalidSQL;
use crate::query::errors::InvalidSQL::Syntax;
use crate::query::structs::expression::ExpressionNode;
use crate::query::structs::operation::Operation;
use crate::query::structs::operation::Operation::{Delete, Insert, Select, Unknown, Update};
use crate::query::structs::ordering::Ordering;
use crate::query::structs::token::Token;
use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};

pub struct Query {
    pub operation: Operation,
    pub table: String,
    pub(crate) columns: Vec<Token>,
    pub(crate) inserts: Vec<Token>,
    pub(crate) updates: Vec<ExpressionNode>,
    pub(crate) conditions: ExpressionNode,
    pub(crate) ordering: Vec<Ordering>,
}

impl Query {
    pub fn default() -> Self {
        Self {
            operation: Unknown,
            table: "".to_string(),
            columns: vec![],
            inserts: vec![],
            updates: vec![],
            conditions: ExpressionNode::default(),
            ordering: vec![],
        }
    }

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

impl Display for Query {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let fields: Vec<&str> = self.columns.iter().map(|f| f.value.as_str()).collect();
        let inserts: Vec<&str> = self.inserts.iter().map(|f| f.value.as_str()).collect();
        writeln!(f, "Query Kind: [{:?}]", self.operation)?;
        writeln!(f, "Table: {:?}", self.table)?;
        writeln!(f, "Columns: {:?}", fields)?;
        writeln!(f, "Inserts: {:?}", inserts)?;
        writeln!(f, "Updates: {:?}", self.updates)?;
        writeln!(f, "Conditions: {:?}", self.conditions)?;
        writeln!(f, "Ordering: {:?}", self.ordering)
    }
}

impl Debug for Query {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self)
    }
}
