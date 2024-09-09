use crate::errored;
use crate::query::structs::expression::{ExpressionNode, ExpressionResult};
use crate::query::structs::query::Query;
use crate::query::structs::token::Token;
use crate::utils::errors::Errored;
use crate::utils::errors::Errored::{Column, Default, Syntax, Table};
use std::collections::HashMap;

/// Representa una fila en una tabla, con un encabezado y valores asociados.
pub struct Row<'a> {
    pub header: &'a Vec<String>,
    pub values: HashMap<String, String>,
}

impl<'a> Row<'a> {
    /// Crea una nueva instancia de `Row` con un encabezado dado.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// use rustic_sql::query::structs::row::Row;
    /// let header = vec!["id".to_string(), "apellido".to_string()];
    /// let row = Row::new(&header);
    /// ```
    pub fn new(header: &'a Vec<String>) -> Self {
        Self {
            header,
            values: HashMap::new(),
        }
    }

    /// Establece un valor para una columna en la fila.
    ///
    /// # Parámetros
    ///
    /// - `key`: El nombre de la columna.
    /// - `value`: El valor a asignar.
    ///
    /// # Errores
    ///
    /// Devuelve un error si la columna no existe en el encabezado.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// use rustic_sql::query::structs::row::Row;
    /// let header = vec!["id".to_string(), "apellido".to_string()];
    /// let mut row = Row::new(&header);
    /// row.set("id", "123".to_string()).unwrap();
    /// ```
    pub fn set(&mut self, key: &str, value: String) -> Result<(), Errored> {
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

    /// Limpia los valores de la fila, estableciendo cada columna con un string vacío.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// use rustic_sql::query::structs::row::Row;
    /// let header = vec!["id".to_string(), "apellido".to_string()];
    /// let mut row = Row::new(&header);
    /// row.set("id", "123".to_string()).unwrap();
    /// row.clear().unwrap();
    /// ```
    pub fn clear(&mut self) -> Result<(), Errored> {
        for key in self.header {
            self.set(key, "".to_string())?
        }
        Ok(())
    }

    /// Aplica una lista de actualizaciones a la fila.
    ///
    /// # Parámetros
    ///
    /// - `updates`: Lista de expresiones que representan las actualizaciones.
    ///
    /// # Errores
    ///
    /// Devuelve un error si ocurre un problema al aplicar las actualizaciones.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// use rustic_sql::query::structs::expression::{ExpressionNode, ExpressionOperator};
    /// use rustic_sql::query::structs::row::Row;
    /// use rustic_sql::query::structs::token::Token;
    /// use rustic_sql::query::structs::token::TokenKind::{Identifier, String};
    /// let header = vec!["id".to_string(), "apellido".to_string()];
    /// let mut row = Row::new(&header);
    /// let update = ExpressionNode::Statement {
    ///     operator: ExpressionOperator::Equals,
    ///     left: Box::new(ExpressionNode::Leaf(Token {
    ///         kind: Identifier,
    ///         value: "id".to_string(),
    ///     })),
    ///     right: Box::new(ExpressionNode::Leaf(Token {
    ///         kind: String,
    ///         value: "360".to_string(),
    ///     })),
    /// };
    /// row.apply_updates(&vec![update]).unwrap();
    /// ```
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

    /// Lee una nueva fila con los valores proporcionados.
    ///
    /// # Parámetros
    ///
    /// - `values`: Valores a insertar en la fila.
    ///
    /// # Errores
    ///
    /// Devuelve un error si el número de valores no coincide con el número de columnas.
    /// Tambien devuelve error si llega a fallar la inserción.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// use rustic_sql::query::structs::row::Row;
    /// let header = vec!["id".to_string(), "apellido".to_string()];
    /// let mut row = Row::new(&header);
    /// let values = vec!["360".to_string(), "katta".to_string()];
    /// row.read_new_row(values).unwrap();
    /// ```
    pub fn read_new_row(&mut self, values: Vec<String>) -> Result<(), Errored> {
        if self.header.len() != values.len() {
            errored!(
                Table,
                "new row has ({}) fields but table needs ({}).",
                values.len(),
                self.header.len()
            );
        }
        for (key, value) in self.header.iter().zip(values) {
            self.set(key, value)?;
        }
        Ok(())
    }

    /// Inserta valores en columnas específicas.
    ///
    /// # Parámetros
    ///
    /// - `columns`: Lista de columnas en las que insertar los valores.
    /// - `values`: Valores a insertar.
    ///
    /// # Errores
    ///
    /// Devuelve un error si alguna columna no existe.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// use rustic_sql::query::structs::row::Row;
    /// use rustic_sql::query::structs::token::Token;
    /// use rustic_sql::query::structs::token::TokenKind::Identifier;
    /// let header = vec!["id".to_string(), "apellido".to_string()];
    /// let mut row = Row::new(&header);
    /// let columns = vec![
    ///     Token {
    ///         kind: Identifier,
    ///         value: "id".to_string(),
    ///     },
    ///     Token {
    ///         kind: Identifier,
    ///         value: "apellido".to_string(),
    ///     },
    /// ];
    /// let values = vec!["360".to_string(), "katta".to_string()];
    /// row.insert_values(&columns, values).unwrap();
    /// ```
    pub fn insert_values(&mut self, columns: &[Token], values: Vec<String>) -> Result<(), Errored> {
        for (col, value) in columns.iter().zip(values) {
            self.set(&col.value, value)?
        }
        Ok(())
    }

    /// Convierte la fila en un string CSV con campos específicos.
    ///
    /// # Parámetros
    ///
    /// - `fields`: Lista de campos a incluir en la proyección CSV.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// use rustic_sql::query::structs::row::Row;
    /// let header = vec!["id".to_string(), "apellido".to_string()];
    /// let mut row = Row::new(&header);
    /// row.set("id", "360".to_string()).unwrap();
    /// row.set("apellido", "katta".to_string()).unwrap();
    /// let csv_string = row.as_csv_projection(&vec!["id".to_string(), "apellido".to_string()]);
    /// assert_eq!(csv_string, "360,katta");
    /// ```
    pub fn as_csv_projection(&self, fields: &Vec<String>) -> String {
        let mut projection: Vec<&str> = Vec::new();
        for key in fields {
            let value = self.values.get(key).map(|v| v.as_str()).unwrap_or("");
            projection.push(value);
        }
        projection.join(",")
    }

    /// Convierte la fila completa en un string CSV.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// use rustic_sql::query::structs::row::Row;
    /// let header = vec!["id".to_string(), "apellido".to_string()];
    /// let mut row = Row::new(&header);
    /// row.set("id", "360".to_string()).unwrap();
    /// row.set("apellido", "katta".to_string()).unwrap();
    /// let csv_string = row.as_csv_row();
    /// assert_eq!(csv_string, "360,katta");
    /// ```
    pub fn as_csv_row(&self) -> String {
        self.as_csv_projection(self.header)
    }

    /// Imprime los valores de la fila según las columnas especificadas.
    ///
    /// # Parámetros
    ///
    /// - `columns`: Lista de columnas para imprimir.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// use rustic_sql::query::structs::row::Row;
    /// use rustic_sql::query::structs::token::Token;
    /// use rustic_sql::query::structs::token::TokenKind::Identifier;
    /// let header = vec!["id".to_string(), "apellido".to_string()];
    /// let mut row = Row::new(&header);
    /// row.set("id", "360".to_string()).unwrap();
    /// row.set("apellido", "katta".to_string()).unwrap();
    /// row.print_projection(&vec![Token { kind: Identifier, value: "id".to_string() }]);
    /// ```
    pub fn print_projection(&self, columns: &[Token]) {
        if columns.is_empty() {
            println!("{}", self.as_csv_row());
        } else {
            let values: Vec<String> = columns.iter().map(|t| t.value.to_string()).collect();
            println!("{}", self.as_csv_projection(&values));
        }
    }

    /// Verifica si la fila cumple con la condición especificada en la consulta.
    ///
    /// Evalúa la condición de la consulta utilizando los valores actuales de la fila.
    /// Si la evaluación resulta en un valor booleano, se devuelve este valor.
    /// Si el resultado no es booleano, se devuelve un error de sintaxis.
    ///
    /// # Parámetros
    ///
    /// - `query`: La consulta que contiene la condición que se debe evaluar.
    ///
    /// # Errores
    ///
    /// Devuelve un error si la evaluación de la condición no resulta en un valor booleano.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// use rustic_sql::query::structs::expression::{ExpressionNode, ExpressionOperator};
    /// use rustic_sql::query::structs::query::Query;
    /// use rustic_sql::query::structs::row::Row;
    /// use rustic_sql::query::structs::token::Token;
    /// use rustic_sql::query::structs::token::TokenKind::{Identifier, Number};
    /// let header = vec!["id".to_string()];
    /// let mut row = Row::new(&header);
    /// row.set("id", "365".to_string()).unwrap();
    ///
    /// let condition = ExpressionNode::Statement {
    ///     operator: ExpressionOperator::GreaterThan,
    ///     left: Box::new(ExpressionNode::Leaf(Token {
    ///         kind: Identifier,
    ///         value: "id".to_string(),
    ///     })),
    ///     right: Box::new(ExpressionNode::Leaf(Token {
    ///         kind: Number,
    ///         value: "360".to_string(),
    ///     })),
    /// };
    ///
    /// let query = Query {
    ///     conditions: condition,
    ///     ..Query::default()
    /// };
    ///
    /// assert!(row.matches_condition(&query).unwrap());
    /// ```
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
    use std::default::Default;

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

        let csv_string = row.as_csv_row();
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
