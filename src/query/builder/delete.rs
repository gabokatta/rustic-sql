use crate::query::builder::{validate_keywords, Builder};
use crate::query::structs::operation::Operation::Delete;
use crate::query::structs::query::Query;
use crate::query::structs::token::Token;
use crate::query::structs::token::TokenKind::Keyword;
use crate::utils::errors::Errored;
use std::collections::VecDeque;

const ALLOWED_KEYWORDS: &[&str] = &["FROM", "WHERE", "AND", "OR"];

/// Constructor para consultas de eliminación (`DELETE`).
///
/// `DeleteBuilder` se encarga de construir una consulta de eliminación a partir
/// de una lista de tokens que representan la sintaxis de una consulta SQL.
pub struct DeleteBuilder {
    tokens: VecDeque<Token>,
}

impl DeleteBuilder {
    /// Crea una nueva instancia de `DeleteBuilder`.
    ///
    /// Este constructor inicializa el `DeleteBuilder` con una cola de tokens
    /// que representan una consulta SQL.
    ///
    /// # Parámetros
    ///
    /// - `tokens`: Cola de tokens (`VecDeque<Token>`) que se utilizarán para construir la consulta.
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self { tokens }
    }
}

impl Builder for DeleteBuilder {
    /// Construye una consulta `DELETE` a partir de los tokens proporcionados.
    ///
    /// Este método analiza los tokens para construir una consulta de eliminación válida.
    /// Verifica palabras clave permitidas, obtiene la tabla objetivo y, opcionalmente,
    /// analiza las condiciones de la cláusula `WHERE`.
    ///
    /// # Retorno
    ///
    /// Retorna un objeto `Query` con la operación de eliminación configurada.
    ///
    /// # Errores
    ///
    /// Retorna un error `Errored` si la consulta contiene errores de sintaxis o palabras clave no permitidas.
    fn build(&mut self) -> Result<Query, Errored> {
        let mut query = Query::default();
        self.validate_keywords()?;
        query.operation = Delete;
        query.table = self.parse_table(Delete)?;
        match self.peek_expecting("WHERE", Keyword) {
            Ok(_) => {
                query.conditions = self.parse_where()?;
            }
            Err(_) => self.expect_none()?,
        }
        Ok(query)
    }

    /// Devuelve una referencia mutable a los tokens de la consulta.
    ///
    /// Este método es utilizado por otros métodos de construcción para acceder y modificar
    /// la lista de tokens durante el proceso de análisis.
    ///
    /// # Retorno
    ///
    /// Retorna una referencia mutable a la cola de tokens (`VecDeque<Token>`).
    fn tokens(&mut self) -> &mut VecDeque<Token> {
        &mut self.tokens
    }

    /// Valida las palabras clave permitidas en una consulta `DELETE`.
    ///
    /// Este método verifica que solo se utilicen las palabras clave permitidas para una
    /// operación de eliminación. Si encuentra alguna palabra clave no permitida, lanza un error.
    ///
    /// Las palabras clave permitidas son: `FROM`, `WHERE`, `AND`, `OR`.
    ///
    /// # Retorno
    ///
    /// Retorna `Ok(())` si todas las palabras clave son válidas.
    ///
    /// # Errores
    ///
    /// Retorna un error `Errored` si se encuentra una palabra clave no válida.
    fn validate_keywords(&self) -> Result<(), Errored> {
        validate_keywords(ALLOWED_KEYWORDS, &self.tokens, Delete)
    }
}

#[cfg(test)]
mod tests {
    use crate::query::structs::expression::ExpressionNode::Empty;
    use crate::query::structs::operation::Operation::Delete;
    use crate::query::structs::query::Query;
    use crate::query::structs::token::Token;
    use crate::query::tokenizer::Tokenizer;

    fn tokenize(sql: &str) -> Vec<Token> {
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(sql).unwrap()
    }

    #[test]
    fn test_delete_simple() {
        let sql = "DELETE FROM ordenes";
        let tokens = tokenize(sql);
        let query = Query::from(tokens).unwrap();

        assert_eq!(query.operation, Delete);
        assert_eq!(query.table, "ordenes");
        assert_eq!(query.conditions, Empty);
    }

    #[test]
    fn test_delete_with_conditions() {
        let sql = "DELETE FROM ordenes WHERE id = 1";
        let tokens = tokenize(sql);
        let query = Query::from(tokens).unwrap();

        assert_eq!(query.operation, Delete);
        assert_eq!(query.table, "ordenes");
        assert_ne!(query.conditions, Empty);
    }

    #[test]
    fn test_delete_invalid_keyword() {
        let sql = "DELETE FROM ordenes ORDER BY id";
        let tokens = tokenize(sql);
        let result = Query::from(tokens);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("ORDER BY"));
    }

    #[test]
    fn test_delete_missing_table() {
        let sql = "DELETE WHERE id = 1";
        let tokens = tokenize(sql);
        let result = Query::from(tokens);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("FROM"));
    }
}
