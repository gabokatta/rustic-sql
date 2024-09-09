use crate::errored;
use crate::query::builder::{unexpected_token_in_stage, validate_keywords, Builder};
use crate::query::structs::operation::Operation::Insert;
use crate::query::structs::query::Query;
use crate::query::structs::token::TokenKind::{Keyword, ParenthesisClose, ParenthesisOpen};
use crate::query::structs::token::{Token, TokenKind};
use crate::utils::errors::Errored;
use crate::utils::errors::Errored::Syntax;
use std::collections::VecDeque;

const ALLOWED_KEYWORDS: &[&str] = &["VALUES"];

/// Estructura `InsertBuilder` que permite construir una consulta de tipo INSERT.
pub struct InsertBuilder {
    tokens: VecDeque<Token>,
}

impl InsertBuilder {
    /// Crea una nueva instancia de `InsertBuilder` con los tokens proporcionados.
    ///
    /// # Parámetros
    /// - `tokens`: Un `VecDeque<Token>` que contiene los tokens de la consulta.
    ///
    /// # Retorna
    /// - Una instancia de `InsertBuilder`.
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self { tokens }
    }

    /// Analiza los valores de inserción de una consulta SQL INSERT.
    ///
    /// Este método espera encontrar la palabra clave `VALUES` seguida de un grupo de valores
    /// entre paréntesis. Los valores pueden ser cadenas o números.
    ///
    /// # Retorna
    /// - Un `Result` que contiene un vector de vectores de `Token` representando los valores de inserción.
    ///
    /// # Errores
    /// - Retorna un error si no se encuentra la palabra clave `VALUES` o si los valores no están
    ///   correctamente formateados.
    fn parse_insert_values(&mut self) -> Result<Vec<Vec<Token>>, Errored> {
        self.pop_expecting("VALUES", Keyword)?;
        self.peek_expecting("(", ParenthesisOpen)?;
        let mut inserts = vec![];
        let mut values = vec![];
        while let Some(t) = self.tokens.front() {
            match t.kind {
                TokenKind::String | TokenKind::Number => {
                    if let Some(token) = self.tokens.pop_front() {
                        values.push(token);
                    }
                }
                ParenthesisOpen => {
                    self.tokens.pop_front();
                }
                ParenthesisClose => {
                    self.tokens.pop_front();
                    inserts.push(values);
                    values = vec![];
                }
                _ => unexpected_token_in_stage("VALUES", t)?,
            }
        }
        Ok(inserts)
    }

    /// Este método asegura que el número de valores en cada inserción coincida con el número
    /// de columnas definidas en la consulta.
    ///
    /// # Parámetros
    /// - `query`: Referencia a la consulta `Query` que contiene las columnas y los valores de inserción.
    ///
    /// # Retorna
    /// - `Ok(())` si los valores son válidos.
    ///
    /// # Errores
    /// - Retorna un error si el número de columnas y el número de valores en la inserción no coinciden.
    fn validate_inserts(&self, query: &Query) -> Result<(), Errored> {
        for insert in &query.inserts {
            let columns = query.columns.len();
            if insert.len() != columns {
                let values: Vec<&String> = insert.iter().map(|t| &t.value).collect();
                errored!(
                    Syntax,
                    "expected {} columns but insert has:\n{:?}",
                    columns,
                    values
                )
            }
        }
        Ok(())
    }
}

impl Builder for InsertBuilder {
    /// Construye una consulta de tipo INSERT a partir de los tokens.
    ///
    /// Este método analiza los tokens, identifica las columnas, los valores a insertar y valida
    /// la estructura de la consulta.
    ///
    /// # Retorna
    /// - Un `Result` que contiene la consulta `Query` si se construye exitosamente.
    ///
    /// # Errores
    /// - Retorna un error si la consulta no está correctamente formada o contiene palabras clave
    ///   inválidas.
    fn build(&mut self) -> Result<Query, Errored> {
        let mut query = Query::default();
        self.validate_keywords()?;
        query.operation = Insert;
        query.table = self.parse_table(Insert)?;
        self.peek_expecting("(", ParenthesisOpen)?;
        query.columns = self.parse_columns()?;
        query.inserts = self.parse_insert_values()?;
        self.expect_none()?;
        self.validate_inserts(&query)?;
        Ok(query)
    }

    /// Retorna una referencia mutable a los tokens que se están procesando.
    ///
    /// # Retorna
    /// - Una referencia mutable a `VecDeque<Token>`.
    fn tokens(&mut self) -> &mut VecDeque<Token> {
        &mut self.tokens
    }

    /// Este método compara las palabras clave en los tokens con las permitidas para asegurarse
    /// de que la consulta sea válida.
    ///
    /// # Retorna
    /// - `Ok(())` si las palabras clave son válidas.
    ///
    /// # Errores
    /// - Retorna un error si se detecta una palabra clave inválida en la consulta.
    fn validate_keywords(&self) -> Result<(), Errored> {
        validate_keywords(ALLOWED_KEYWORDS, &self.tokens, Insert)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::structs::token::TokenKind::{Identifier, Number, String};
    use crate::query::tokenizer::Tokenizer;

    fn tokenize(sql: &str) -> Vec<Token> {
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(sql).unwrap()
    }

    fn to_token(value: &str, kind: TokenKind) -> Token {
        Token {
            value: value.to_string(),
            kind,
        }
    }

    #[test]
    fn test_insert_simple() {
        let sql = "INSERT INTO ordenes (id, producto) VALUES (1, 'Laptop')";
        let tokens = tokenize(sql);

        let query = Query::from(tokens).unwrap();

        assert_eq!(query.operation, Insert);
        assert_eq!(query.table, "ordenes");
        assert_eq!(
            query.columns,
            vec![to_token("id", Identifier), to_token("producto", Identifier)]
        );
        assert_eq!(
            query.inserts,
            vec![vec![to_token("1", Number), to_token("Laptop", String),]]
        );
    }

    #[test]
    fn test_insert_multiple_values() {
        let sql = "INSERT INTO ordenes (id, producto) VALUES (1, 'Laptop'), (2, 'PS4');";
        let tokens = tokenize(sql);
        let query = Query::from(tokens).unwrap();

        assert_eq!(query.operation, Insert);
        assert_eq!(query.table, "ordenes");
        assert_eq!(
            query.columns,
            vec![to_token("id", Identifier), to_token("producto", Identifier)]
        );
        assert_eq!(
            query.inserts,
            vec![
                vec![to_token("1", Number), to_token("Laptop", String),],
                vec![to_token("2", Number), to_token("PS4", String),]
            ]
        );
    }

    #[test]
    fn test_insert_invalid_columns() {
        let sql = "INSERT INTO ordenes (id, producto) VALUES (1)";
        let tokens = tokenize(sql);
        let result = Query::from(tokens);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("expected 2 columns"));
    }

    #[test]
    fn test_insert_invalid_keyword() {
        let sql = "INSERT INTO ordenes (id, producto) VALUES (1, 'LAPTOP') WHERE 1=1";
        let tokens = tokenize(sql);
        let result = Query::from(tokens);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("WHERE"));
    }

    #[test]
    fn test_insert_invalid_missing_parenthesis() {
        let sql = "INSERT INTO ordenes id, producto VALUES (1, 'LAPTOP')";
        let tokens = tokenize(sql);
        let result = Query::from(tokens);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected"));
    }
}
