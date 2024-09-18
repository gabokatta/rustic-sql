use crate::query::builder::{unexpected_token_in_stage, validate_keywords, Builder};
use crate::query::structs::operation::Operation::Select;
use crate::query::structs::ordering::OrderKind::{Asc, Desc};
use crate::query::structs::ordering::Ordering;
use crate::query::structs::query::Query;
use crate::query::structs::token::Token;
use crate::query::structs::token::TokenKind::{Identifier, Keyword};
use crate::utils::errors::Errored;
use std::collections::VecDeque;

const ALLOWED_KEYWORDS: &[&str] = &[
    "SELECT", "FROM", "WHERE", "ORDER BY", "ASC", "DESC", "AND", "OR", "NOT",
];

/// Esta estructura procesa los tokens de una consulta SQL y construye una consulta SELECT
/// con las columnas, la tabla, las condiciones y el orden especificados.
pub struct SelectBuilder {
    tokens: VecDeque<Token>,
}

impl SelectBuilder {
    /// Crea una nueva instancia de `SelectBuilder` con los tokens proporcionados.
    ///
    /// # Parámetros
    /// - `tokens`: Un `VecDeque<Token>` que contiene los tokens de la consulta.
    ///
    /// # Retorna
    /// - Una instancia de `SelectBuilder`.
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self { tokens }
    }

    /// Analiza y extrae las expresiones de ordenamiento de la consulta.
    ///
    /// Este método procesa los tokens después de la cláusula `ORDER BY` y construye
    /// una lista de ordenamientos basados en los campos y la dirección (ASC o DESC).
    ///
    /// # Retorna
    /// - Un `Result` que contiene un vector de `Ordering` representando las expresiones de ordenamiento.
    ///
    /// # Errores
    /// - Retorna un error si se encuentra un token inesperado en la fase de ordenamiento.
    fn parse_ordering(&mut self) -> Result<Vec<Ordering>, Errored> {
        let mut ordering = vec![];
        while let Some(t) = self.tokens.pop_front() {
            if t.kind != Identifier {
                unexpected_token_in_stage("ORDER_BY", &t)?
            }
            let mut new_order = Ordering {
                field: t,
                ..Ordering::default()
            };
            if let Some(next) = self.tokens.front() {
                match next.kind {
                    Keyword if next.value == "ASC" || next.value == "DESC" => {
                        new_order.kind = if next.value == "DESC" { Desc } else { Asc };
                        self.tokens.pop_front();
                    }
                    _ => {}
                }
            }
            ordering.push(new_order)
        }
        Ok(ordering)
    }
}

impl Builder for SelectBuilder {
    /// Construye una consulta de tipo SELECT a partir de los tokens.
    ///
    /// Este método analiza los tokens para identificar las columnas, la tabla,
    /// las condiciones y las expresiones de ordenamiento de la consulta.
    ///
    /// # Retorna
    /// - Un `Result` que contiene la consulta `Query` si se construye exitosamente.
    ///
    /// # Errores
    /// - Retorna un error si la consulta no está correctamente formada o si se detectan
    ///   palabras clave inválidas.
    fn build(&mut self) -> Result<Query, Errored> {
        let mut query = Query::default();
        self.validate_keywords()?;
        query.operation = Select;
        query.columns = self.parse_columns()?;
        query.table = self.parse_table(Select)?;
        if self.peek_expecting("WHERE", Keyword).is_ok() {
            query.conditions = self.parse_where()?;
        }
        match self.peek_expecting("ORDER BY", Keyword) {
            Ok(_) => {
                self.tokens.pop_front();
                query.ordering = self.parse_ordering()?;
            }
            Err(_) => self.expect_none()?,
        }
        Ok(query)
    }

    /// Retorna una referencia mutable a los tokens que se están procesando.
    ///
    /// # Retorna
    /// - Una referencia mutable a `VecDeque<Token>`.
    fn tokens(&mut self) -> &mut VecDeque<Token> {
        &mut self.tokens
    }

    /// Valida que las palabras clave usadas en la consulta sean válidas para una consulta SELECT.
    ///
    /// Este método compara las palabras clave en los tokens con las permitidas para asegurarse
    /// de que la consulta sea válida.
    ///
    /// # Retorna
    /// - `Ok(())` si las palabras clave son válidas.
    ///
    /// # Errores
    /// - Retorna un error si se detecta una palabra clave inválida en la consulta.
    fn validate_keywords(&self) -> Result<(), Errored> {
        validate_keywords(ALLOWED_KEYWORDS, &self.tokens, Select)
    }
}

#[cfg(test)]
mod tests {
    use crate::query::structs::expression::ExpressionNode::Empty;
    use crate::query::structs::operation::Operation::Select;
    use crate::query::structs::ordering::OrderKind::{Asc, Desc};
    use crate::query::structs::ordering::Ordering;
    use crate::query::structs::query::Query;
    use crate::query::structs::token::TokenKind::Identifier;
    use crate::query::structs::token::{Token, TokenKind};
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
    fn test_select_basic() {
        let sql = "SELECT id, producto FROM ordenes";
        let tokens = tokenize(sql);
        let query = Query::from(tokens).unwrap();

        assert_eq!(query.operation, Select);
        assert_eq!(
            query.columns,
            vec![to_token("id", Identifier), to_token("producto", Identifier),]
        );
        assert_eq!(query.table, "ordenes");
        assert_eq!(query.conditions, Empty);
        assert!(query.ordering.is_empty());
    }

    #[test]
    fn test_select_with_conditions() {
        let sql = "SELECT id, producto, cantidad FROM ordenes WHERE cantidad > 30";
        let tokens = tokenize(sql);
        let query = Query::from(tokens).unwrap();

        assert_eq!(query.operation, Select);
        assert_eq!(
            query.columns,
            vec![
                to_token("id", Identifier),
                to_token("producto", Identifier),
                to_token("cantidad", Identifier),
            ]
        );
        assert_eq!(query.table, "ordenes");
        assert_ne!(query.conditions, Empty);
        assert!(query.ordering.is_empty());
    }

    #[test]
    fn test_select_with_ordering() {
        let sql = "SELECT id, producto FROM ordenes ORDER BY id DESC, producto";
        let tokens = tokenize(sql);
        let query = Query::from(tokens).unwrap();

        assert_eq!(query.operation, Select);
        assert_eq!(
            query.columns,
            vec![to_token("id", Identifier), to_token("producto", Identifier),]
        );
        assert_eq!(query.table, "ordenes");
        assert_eq!(query.conditions, Empty);
        assert_eq!(query.ordering.len(), 2);
        assert_eq!(
            query.ordering[0],
            Ordering {
                field: to_token("id", Identifier),
                kind: Desc,
            }
        );
        assert_eq!(
            query.ordering[1],
            Ordering {
                field: to_token("producto", Identifier),
                kind: Asc,
            }
        );
    }

    #[test]
    fn test_select_invalid_keyword() {
        let sql = "SELECT id, name FROM users ORDER BY id DESC VALUES";
        let tokens = tokenize(sql);
        let result = Query::from(tokens);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("VALUES"));
    }

    #[test]
    fn test_select_missing_from() {
        let sql = "SELECT id, name users";
        let tokens = tokenize(sql);
        let result = Query::from(tokens);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("FROM"));
    }
}
