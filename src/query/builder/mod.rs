pub mod delete;
pub mod expression;
pub mod insert;
pub mod select;
pub mod update;

use crate::errored;
use crate::query::builder::expression::ExpressionBuilder;
use crate::query::structs::expression::ExpressionNode;
use crate::query::structs::operation::Operation;
use crate::query::structs::operation::Operation::{Delete, Insert, Select, Unknown, Update};
use crate::query::structs::query::Query;
use crate::query::structs::token::TokenKind::{
    Identifier, Keyword, Operator, ParenthesisClose, ParenthesisOpen,
};
use crate::query::structs::token::{Token, TokenKind};
use crate::utils::errors::Errored;
use crate::utils::errors::Errored::*;
use std::collections::VecDeque;

/// Interfaz para construir y validar una consulta SQL.
///
/// Este trait define los métodos necesarios para analizar y construir consultas SQL,
/// incluyendo la validación de las palabras clave y la extracción de tokens relacionados
/// con tablas, columnas y condiciones.
pub trait Builder {
    /// Construye y retorna una consulta SQL (`Query`) basada en los tokens disponibles.
    ///
    /// # Errores
    ///
    /// Retorna un error de tipo `Errored` si hay problemas al construir la consulta.
    fn build(&mut self) -> Result<Query, Errored>;

    /// Retorna una referencia mutable a la cola de tokens (`VecDeque<Token>`).
    ///
    /// # Retorno
    ///
    /// Una referencia mutable a los tokens procesados por el builder.
    fn tokens(&mut self) -> &mut VecDeque<Token>;

    /// Analiza el nombre de la tabla de una consulta SQL.
    ///
    /// Este método maneja las palabras clave `FROM` para consultas `SELECT` y `DELETE`.
    /// En el caso de otras Operaciones SQL, no hay ningun tipo de token que se interponga entre
    /// las palabras claves y la tabla, por esto, se saltan.
    ///
    /// # Parámetros
    ///
    /// - `operation`: Operación SQL (`Select`, `Delete`, etc.) que define cómo se analiza la tabla.
    ///
    /// # Retorno
    ///
    /// Retorna el nombre de la tabla como `String`.
    ///
    /// # Errores
    ///
    /// Retorna un error `Errored` si no se encuentra un identificador de tabla válido.
    fn parse_table(&mut self, operation: Operation) -> Result<String, Errored> {
        match operation {
            Select | Delete => {
                self.peek_expecting("FROM", Keyword)?;
                self.tokens().pop_front();
            }
            _ => {}
        }
        let t = self
            .tokens()
            .pop_front()
            .ok_or_else(|| Syntax("could not find table identifier.".to_string()))?;
        if t.kind != Identifier {
            unexpected_token_in_stage("TABLE", &t)?;
        }
        Ok(t.value)
    }

    /// Analiza las columnas de una consulta SQL.
    ///
    /// Este método procesa los tokens que representan columnas en las cláusulas `SELECT` o `INSERT`.
    ///
    /// # Retorno
    ///
    /// Retorna un vector de tokens que representan las columnas de la consulta.
    ///
    /// # Errores
    ///
    /// Retorna un error `Errored` si se encuentra un token inesperado durante el análisis de columnas.
    fn parse_columns(&mut self) -> Result<Vec<Token>, Errored> {
        let mut fields: Vec<Token> = vec![];
        while let Some(t) = self.tokens().front() {
            match t.kind {
                Identifier => {
                    if let Some(op) = self.tokens().pop_front() {
                        fields.push(op);
                    }
                }
                Keyword if t.value == "FROM" || t.value == "VALUES" => {
                    if fields.is_empty() {
                        errored!(Syntax, "read FROM without any * or fields in query.")
                    }
                    break;
                }
                ParenthesisClose => {
                    self.tokens().pop_front();
                    break;
                }
                Operator if t.value == "*" => {
                    self.tokens().pop_front();
                    break;
                }
                ParenthesisOpen => {
                    self.tokens().pop_front();
                }
                _ => unexpected_token_in_stage("COLUMN", t)?,
            }
        }
        Ok(fields)
    }

    /// Analiza la cláusula `WHERE` en una consulta SQL.
    ///
    /// Este método busca la palabra clave `WHERE` y luego construye la expresión correspondiente.
    ///
    /// # Retorno
    ///
    /// Retorna un nodo de expresión (`ExpressionNode`) que representa la condición `WHERE`.
    ///
    /// # Errores
    ///
    /// Retorna un error `Errored` si no se encuentra o se procesa incorrectamente la cláusula `WHERE`.
    fn parse_where(&mut self) -> Result<ExpressionNode, Errored> {
        self.pop_expecting("WHERE", Keyword)?;
        ExpressionBuilder::parse_expressions(self.tokens())
    }

    /// Valida que no haya más tokens después de la consulta.
    ///
    /// # Errores
    ///
    /// Retorna un error `Errored` si se encuentran tokens adicionales al final de la consulta.
    fn expect_none(&mut self) -> Result<(), Errored> {
        if let Some(t) = self.tokens().front() {
            errored!(Syntax, "expected end of query but got: {:?}", t);
        }
        Ok(())
    }

    /// Extrae el siguiente token si coincide con el valor y tipo esperados.
    ///
    /// # Parámetros
    ///
    /// - `value`: El valor esperado del token.
    /// - `kind`: El tipo esperado del token (`TokenKind`).
    ///
    /// # Retorno
    ///
    /// Retorna el token extraído si cumple con las expectativas, o `None` si no.
    ///
    /// # Errores
    ///
    /// Retorna un error `Errored` si no se encuentra el token esperado.
    fn pop_expecting(&mut self, value: &str, kind: TokenKind) -> Result<Option<Token>, Errored> {
        self.peek_expecting(value, kind)?;
        Ok(self.tokens().pop_front())
    }

    /// Verifica si el siguiente token en la lista coincide con el valor y tipo esperados.
    ///
    /// # Parámetros
    ///
    /// - `value`: El valor esperado del token.
    /// - `kind`: El tipo esperado del token (`TokenKind`).
    ///
    /// # Errores
    ///
    /// Retorna un error `Errored` si no se encuentra el token esperado.
    fn peek_expecting(&mut self, value: &str, kind: TokenKind) -> Result<(), Errored> {
        let expected = Token {
            value: value.to_string(),
            kind,
        };
        if let Some(t) = self.tokens().front() {
            if t.kind != expected.kind || t.value != expected.value.to_uppercase() {
                errored!(Syntax, "expected {:?} token, got: {:?}", expected, t)
            }
        } else {
            errored!(Syntax, "got None when expecting: {:?}", expected)
        }
        Ok(())
    }

    /// Valida que solo se usen palabras clave permitidas en la consulta.
    ///
    /// # Errores
    ///
    /// Retorna un error `Errored` si se encuentra una palabra clave inválida.
    fn validate_keywords(&self) -> Result<(), Errored>;
}

/// Determina el tipo de operación SQL a partir de un token.
///
/// # Parámetros
///
/// - `token`: Un token opcional que representa una palabra clave de operación SQL.
///
/// # Retorno
///
/// Retorna el tipo de operación (`Operation`), como `Select`, `Insert`, `Update`, `Delete`, o `Unknown` si no se reconoce la palabra clave.
pub fn get_kind(token: Option<Token>) -> Operation {
    match token {
        Some(t) => match t.value.as_str() {
            "SELECT" => Select,
            "INSERT INTO" => Insert,
            "UPDATE" => Update,
            "DELETE" => Delete,
            _ => Unknown,
        },
        None => Unknown,
    }
}

/// Valida que solo se usen palabras clave permitidas en una consulta SQL.
///
/// # Parámetros
///
/// - `allowed`: Un arreglo de palabras clave permitidas.
/// - `tokens`: Una lista de tokens procesados.
/// - `operation`: El tipo de operación SQL para la validación.
///
/// # Retorno
///
/// Retorna un `Result` indicando si la validación fue exitosa.
///
/// # Errores
///
/// Retorna un error `Errored` si se encuentra una palabra clave no permitida.
fn validate_keywords(
    allowed: &[&str],
    tokens: &VecDeque<Token>,
    operation: Operation,
) -> Result<(), Errored> {
    let keywords: VecDeque<&Token> = tokens.iter().filter(|t| t.kind == Keyword).collect();
    for word in keywords {
        if !allowed.contains(&&*word.value) {
            errored!(
                Syntax,
                "invalid keyword for {:?} query detected: {}",
                operation,
                word.value
            )
        }
    }
    Ok(())
}

/// Lanza un error cuando se encuentra un token inesperado durante el análisis de una consulta.
///
/// # Parámetros
///
/// - `stage`: El nombre de la etapa de análisis en la que se encontró el token.
/// - `token`: El token inesperado encontrado.
///
/// # Retorno
///
/// Retorna un `Result` con un error `Errored`.
pub fn unexpected_token_in_stage(stage: &str, token: &Token) -> Result<(), Errored> {
    errored!(
        Syntax,
        "unexpected token while parsing {} fields: {:?}",
        stage,
        token
    )
}
