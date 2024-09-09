use crate::errored;
use crate::query::structs::expression::{ExpressionNode, ExpressionResult};
use crate::query::structs::query::Query;
use crate::query::structs::token::Token;
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

    fn set(&mut self, key: &str, value: String) -> Result<(), Errored> {
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

    pub fn clear(&mut self) -> Result<(), Errored> {
        for key in self.header {
            self.set(key, "".to_string())?
        }
        Ok(())
    }

    pub fn apply_updates(&mut self, updates: &Vec<ExpressionNode>) -> Result<(), Errored> {
        for up in updates {
            if let Ok((field, value)) = up.as_leaf_tuple() {
                let k = &field.value;
                let v = &value.value;
                self.set(k, v.to_string())?
            } else {
                errored!(Default, "error while updating values.")
            }
        }
        Ok(())
    }

    pub fn read_new_row(&mut self, values: Vec<String>) -> Result<(), Errored> {
        if self.header.len() != values.len() {
            errored!(
                Table,
                "new row ({}) has less fields than table needs ({}).",
                values.len(),
                self.header.len()
            );
        }
        for (key, value) in self.header.iter().zip(values) {
            self.set(key, value)?;
        }
        Ok(())
    }

    pub fn insert_values(&mut self, columns: &[Token], values: Vec<String>) -> Result<(), Errored> {
        for (col, value) in columns.iter().zip(values) {
            self.set(&col.value, value)?
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::structs::expression::{ExpressionNode, ExpressionOperator};
    use crate::query::structs::token::Token;
    use crate::query::structs::token::TokenKind::*;

    #[test]
    fn test_initializing() {
        let header = vec!["id".to_string(), "apellido".to_string()];
        let row = Row::new(&header);
        assert_eq!(row.header, &header);
        assert_eq!(row.values.len(), 0);
    }

    #[test]
    fn test_insert_valid_column() {
        let header = vec!["id".to_string(), "apellido".to_string()];
        let mut row = Row::new(&header);
        assert!(row.set("id", "123".to_string()).is_ok());
        assert_eq!(row.values.get("id").unwrap(), "123");
    }

    #[test]
    fn test_insert_invalid_column() {
        let header = vec!["id".to_string(), "apellido".to_string()];
        let mut row = Row::new(&header);
        assert!(row.set("nombre", "gabriel".to_string()).is_err());
    }

    #[test]
    fn test_clear() {
        let header = vec!["id".to_string(), "apellido".to_string()];
        let mut row = Row::new(&header);
        row.set("id", "123".to_string()).unwrap();
        row.clear().unwrap();
        assert_eq!(row.values.get("id").unwrap(), "");
        assert_eq!(row.values.get("apellido").unwrap(), "");
    }

    #[test]
    fn test_apply_updates() {
        let header = vec!["id".to_string(), "apellido".to_string()];
        let mut row = Row::new(&header);

        let field = Token {
            kind: Identifier,
            value: "id".to_string(),
        };
        let value = Token {
            kind: String,
            value: "360".to_string(),
        };
        let update = ExpressionNode::Statement {
            operator: ExpressionOperator::Equals,
            left: Box::new(ExpressionNode::Leaf(field)),
            right: Box::new(ExpressionNode::Leaf(value)),
        };

        row.apply_updates(&vec![update]).unwrap();
        assert_eq!(row.values.get("id").unwrap(), "360");
    }

    #[test]
    fn test_read_new_values() {
        let header = vec!["id".to_string(), "apellido".to_string()];
        let mut row = Row::new(&header);
        let values = vec!["360".to_string(), "katta".to_string()];

        row.read_new_row(values).unwrap();
        assert_eq!(row.values.get("id").unwrap(), "360");
        assert_eq!(row.values.get("apellido").unwrap(), "katta");
    }

    #[test]
    fn test_read_new_values_mismatch_length() {
        let header = vec!["id".to_string(), "apellido".to_string()];
        let mut row = Row::new(&header);
        let values = vec!["365".to_string()]; // only one value instead of two
        assert!(row.read_new_row(values).is_err());
    }

    #[test]
    fn test_insert_values() {
        let header = vec!["id".to_string(), "apellido".to_string()];
        let mut row = Row::new(&header);
        let columns = vec![
            Token {
                kind: Identifier,
                value: "id".to_string(),
            },
            Token {
                kind: Identifier,
                value: "apellido".to_string(),
            },
        ];
        let values = vec!["360".to_string(), "katta".to_string()];

        row.insert_values(&columns, values).unwrap();
        assert_eq!(row.values.get("id").unwrap(), "360");
        assert_eq!(row.values.get("apellido").unwrap(), "katta");
    }

    #[test]
    fn test_as_csv_string() {
        let header = vec!["id".to_string(), "apellido".to_string()];
        let mut row = Row::new(&header);
        row.set("id", "360".to_string()).unwrap();
        row.set("apellido", "katta".to_string()).unwrap();

        let csv_string = row.as_csv_string();
        assert_eq!(csv_string, "360,katta");
    }

    #[test]
    fn test_matches_condition() {
        let header = vec!["id".to_string()];
        let mut row = Row::new(&header);
        row.set("id", "365".to_string()).unwrap();
        let condition = ExpressionNode::Statement {
            operator: ExpressionOperator::GreaterThan,
            left: Box::new(ExpressionNode::Leaf(Token {
                kind: Identifier,
                value: "id".to_string(),
            })),
            right: Box::new(ExpressionNode::Leaf(Token {
                kind: Number,
                value: "360".to_string(),
            })),
        };
        let query = Query {
            conditions: condition,
            ..Query::default()
        };
        assert!(row.matches_condition(&query).unwrap());
    }
}
