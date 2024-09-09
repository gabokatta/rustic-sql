use crate::errored;
use crate::query::builder::delete::DeleteBuilder;
use crate::query::builder::insert::InsertBuilder;
use crate::query::builder::select::SelectBuilder;
use crate::query::builder::update::UpdateBuilder;
use crate::query::builder::{get_kind, Builder};
use crate::query::structs::expression::ExpressionNode;
use crate::query::structs::operation::Operation;
use crate::query::structs::operation::Operation::{Delete, Insert, Select, Unknown, Update};
use crate::query::structs::ordering::Ordering;
use crate::query::structs::token::Token;
use crate::utils::errors::Errored;
use crate::utils::errors::Errored::Syntax;
use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};

pub struct Query {
    pub operation: Operation,
    pub table: String,
    pub columns: Vec<Token>,
    pub inserts: Vec<Vec<Token>>,
    pub updates: Vec<ExpressionNode>,
    pub conditions: ExpressionNode,
    pub ordering: Vec<Ordering>,
}

impl Query {
    pub fn from(tokens: Vec<Token>) -> Result<Self, Errored> {
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

impl Default for Query {
    fn default() -> Self {
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
}

impl Display for Query {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let fields: Vec<&str> = self.columns.iter().map(|f| f.value.as_str()).collect();
        writeln!(f, "Query Kind: [{:?}]", self.operation)?;
        writeln!(f, "Table: {:?}", self.table)?;
        writeln!(f, "Columns: {:?}", fields)?;
        writeln!(f, "Inserts {{ ")?;
        for insert in &self.inserts {
            let values: Vec<&String> = insert.iter().map(|t| &t.value).collect();
            writeln!(f, "   {:?}", values)?;
        }
        writeln!(f, "}} ")?;
        writeln!(f, "Updates {{ ")?;
        for up in &self.updates {
            if let Ok((l, r)) = up.as_leaf_tuple() {
                writeln!(f, "   {} -> {}", l.value, r.value)?;
            }
        }
        writeln!(f, "}} ")?;
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
