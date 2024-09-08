use crate::errored;
use crate::query::structs::expression::{ExpressionNode, ExpressionResult};
use crate::query::structs::query::Query;
use crate::utils::errors::Errored;
use crate::utils::errors::Errored::{Column, Default, Syntax, Table};
use std::collections::HashMap;

pub struct Row<'a> {
    pub header: &'a Vec<String>,
    pub values: HashMap<String, String>,
}

impl<'a> Row<'a> {
    pub fn new(header: &'a Vec<String>) -> Self {
        Self {
            header,
            values: HashMap::new(),
        }
    }

    fn insert(&mut self, key: &str, value: String) -> Result<(), Errored> {
        if self.header.contains(&key.to_string()) {
            self.values.insert(key.to_string(), value);
        } else {
            errored!(
                Column,
                "column {} does not exist in table with fields: {:?}",
                key,
                self.header
            )
        }
        Ok(())
    }

    pub fn update_values(&mut self, updates: &Vec<ExpressionNode>) -> Result<(), Errored> {
        for up in updates {
            if let Ok((field, value)) = up.as_leaf_tuple() {
                let k = &field.value;
                let v = &value.value;
                self.insert(k, v.to_string())?
            } else {
                errored!(Default, "error while updating values.")
            }
        }
        Ok(())
    }

    pub fn set_new_values(&mut self, values: Vec<String>) -> Result<(), Errored> {
        if self.header.len() != values.len() {
            errored!(
                Table,
                "new row ({}) has less fields than table needs ({}).",
                values.len(),
                self.header.len()
            );
        }
        for (key, value) in self.header.iter().zip(values) {
            self.insert(key, value)?;
        }
        Ok(())
    }

    pub fn as_csv_string(&self) -> String {
        let mut fields: Vec<&str> = Vec::new();
        for key in self.header {
            let value = self.values.get(key).map(|v| v.as_str()).unwrap_or("");
            fields.push(value);
        }
        fields.join(",")
    }

    pub fn print_values(&self) {
        println!("{}", self.as_csv_string());
    }

    pub fn matches_condition(&self, query: &Query) -> Result<bool, Errored> {
        match query.conditions.evaluate(&self.values)? {
            ExpressionResult::Bool(b) => Ok(b),
            _ => errored!(Syntax, "query condition evaluates to non-boolean value."),
        }
    }
}
