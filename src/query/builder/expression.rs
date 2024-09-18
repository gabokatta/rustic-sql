use crate::errored;
use crate::query::structs::expression::ExpressionNode::{Empty, Leaf};
use crate::query::structs::expression::ExpressionOperator::*;
use crate::query::structs::expression::{ExpressionNode, ExpressionOperator};
use crate::query::structs::token::TokenKind::Keyword;
use crate::query::structs::token::{Token, TokenKind};
use crate::utils::errors::Errored;
use crate::utils::errors::Errored::Syntax;
use std::collections::VecDeque;

/// Estructura para analizar y construir expresiones lógicas en una consulta.
///
/// `ExpressionBuilder` proporciona métodos recursivos para analizar expresiones condicionales
/// como `AND`, `OR`, `NOT`, y comparaciones simples. Estas expresiones son comúnmente
/// utilizadas en las cláusulas `WHERE` de las consultas SQL.
pub struct ExpressionBuilder;

impl ExpressionBuilder {
    /// Inicia el análisis de las expresiones a partir de una lista de tokens.
    ///
    /// Este método comienza el proceso de parseo llamando a `parse_or` para
    /// manejar operaciones lógicas.
    ///
    /// La razón por la cual este método arranca parseando las operaciones OR, es porque
    /// es la que tiene menor precedencia, de esta manera la expresión sera parseada en el siguiente orden de
    /// predecencia:
    ///
    /// - OR -> AND -> NOT -> (Parantesis)
    ///
    /// Teniendo prioridad los operadores que estan más la derecha.
    ///
    /// Podemos hacer esto gracias a la naturaleza recursiva de los métodos que siempre buscaran llegar
    /// a una hoja desde arriba e ir evaluando en reversa hasta retornar.
    ///
    /// # Parámetros
    ///
    /// - `tokens`: Cola de tokens (`VecDeque<Token>`) que representan la consulta a analizar.
    ///
    /// # Retorno
    ///
    /// Retorna un nodo de expresión (`ExpressionNode`) que representa la expresión completa.
    ///
    /// # Errores
    ///
    /// Retorna un error `Errored` si ocurre algún problema durante el análisis.
    pub fn parse_expressions(tokens: &mut VecDeque<Token>) -> Result<ExpressionNode, Errored> {
        ExpressionBuilder::parse_or(tokens)
    }

    /// Analiza las expresiones con el operador `OR`.
    ///
    /// Este método evalúa si existen múltiples expresiones unidas por el operador `OR`
    /// y las agrupa en un nodo de expresión.
    ///
    /// Primero el método busca mediante el parseo de una operacion AND el valor de la rama
    /// izquierda de la expresión actual.
    ///
    /// Si el tóken actual es un operador "OR", consumimos el token y buscamos el valor
    /// de la derecha.
    ///
    /// # Parámetros
    ///
    /// - `tokens`: Cola de tokens a analizar.
    ///
    /// # Retorno
    ///
    /// Retorna un nodo de expresión con operadores `OR`.
    ///
    /// # Errores
    ///
    /// Retorna un error si hay un problema con los tokens.
    fn parse_or(tokens: &mut VecDeque<Token>) -> Result<ExpressionNode, Errored> {
        let mut left = ExpressionBuilder::parse_and(tokens)?;
        while let Some(t) = tokens.front() {
            if t.kind != Keyword || t.value != "OR" {
                break;
            }
            tokens.pop_front();
            let right = ExpressionBuilder::parse_and(tokens)?;
            left = ExpressionNode::Statement {
                operator: Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// Analiza las expresiones con el operador `AND`.
    ///
    /// Similar a `parse_or`, pero para operaciones con `AND`.
    /// Podemos ver como este método le delega a los operadores `NOT` la responsabilidad de evaluar.
    ///
    /// # Parámetros
    ///
    /// - `tokens`: Cola de tokens a analizar.
    ///
    /// # Retorno
    ///
    /// Retorna un nodo de expresión con operadores `AND`.
    ///
    /// # Errores
    ///
    /// Retorna un error si hay un problema con los tokens.
    fn parse_and(tokens: &mut VecDeque<Token>) -> Result<ExpressionNode, Errored> {
        let mut left = ExpressionBuilder::parse_not(tokens)?;
        while let Some(t) = tokens.front() {
            if t.kind != Keyword || t.value != "AND" {
                break;
            }
            tokens.pop_front();
            let right = ExpressionBuilder::parse_not(tokens)?;
            left = ExpressionNode::Statement {
                operator: And,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// Analiza las expresiones con el operador `NOT`.
    ///
    /// Maneja expresiones que comienzan con `NOT`, invirtiendo la lógica de la condición.
    ///
    /// Es el operador booleano con mayor precedencia, en caso de no estar dentro de una operación
    /// NOT, este método se encarga de empezar a estudiar los nodos más simples ya que ninguno ha matcheado hasta
    /// ahora.
    ///
    /// # Parámetros
    ///
    /// - `tokens`: Cola de tokens a analizar.
    ///
    /// # Retorno
    ///
    /// Retorna un nodo de expresión con el operador `NOT`.
    ///
    /// # Errores
    ///
    /// Retorna un error si hay un problema con los tokens.
    fn parse_not(tokens: &mut VecDeque<Token>) -> Result<ExpressionNode, Errored> {
        if let Some(t) = tokens.front() {
            if t.kind == Keyword && t.value == "NOT" {
                tokens.pop_front();
                let node = ExpressionBuilder::parse_comparisons(tokens)?;
                return Ok(ExpressionNode::Statement {
                    operator: Not,
                    left: Box::new(node),
                    right: Box::new(Empty),
                });
            }
        }
        ExpressionBuilder::parse_comparisons(tokens)
    }

    /// Analiza las comparaciones simples en las expresiones.
    ///
    /// Este método procesa las comparaciones entre dos valores usando operadores como `=`, `>`, `<`, etc.
    ///
    /// # Parámetros
    ///
    /// - `tokens`: Cola de tokens a analizar.
    ///
    /// # Retorno
    ///
    /// Retorna un nodo de expresión que representa una comparación.
    ///
    /// # Errores
    ///
    /// Retorna un error si los tokens no forman una comparación válida.
    fn parse_comparisons(tokens: &mut VecDeque<Token>) -> Result<ExpressionNode, Errored> {
        let left = ExpressionBuilder::parse_leaf(tokens)?;
        let operator = ExpressionBuilder::parse_simple_operator(tokens);
        if operator.is_err() {
            return Ok(left);
        }
        let right = ExpressionBuilder::parse_leaf(tokens)?;
        Ok(ExpressionNode::Statement {
            operator: operator?,
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    /// Analiza las hojas de una expresión, como identificadores, números o cadenas.
    ///
    /// Este método maneja elementos básicos que no son operadores lógicos, como los valores literales.
    ///
    /// En caso de conseguir un parentesis, es indicador de que una nueva expresión se debe evaluar.
    /// Es acá cuando la recursividad vuelve a empezar.
    ///
    /// # Parámetros
    ///
    /// - `tokens`: Cola de tokens a analizar.
    ///
    /// # Retorno
    ///
    /// Retorna un nodo de expresión que representa una hoja.
    ///
    /// # Errores
    ///
    /// Retorna un error si no se encuentra una hoja válida.
    fn parse_leaf(tokens: &mut VecDeque<Token>) -> Result<ExpressionNode, Errored> {
        let mut leaf = Empty;
        while let Some(t) = tokens.front() {
            match t.kind {
                TokenKind::Identifier | TokenKind::Number | TokenKind::String => {
                    if let Some(t) = tokens.pop_front() {
                        leaf = Leaf(t);
                        break;
                    }
                }
                TokenKind::ParenthesisOpen => {
                    tokens.pop_front();
                    leaf = ExpressionBuilder::parse_expressions(tokens)?;
                }
                TokenKind::ParenthesisClose if leaf != Empty => {
                    tokens.pop_front();
                    break;
                }
                _ => errored!(Syntax, "unexpected token when parsing leaf: {:?}", t),
            }
        }
        Ok(leaf)
    }

    /// Analiza los operadores simples en las comparaciones.
    ///
    /// Este método reconoce operadores como `=`, `!=`, `>`, `<`, etc.
    ///
    /// # Parámetros
    ///
    /// - `tokens`: Cola de tokens a analizar.
    ///
    /// # Retorno
    ///
    /// Retorna el operador de la comparación (`ExpressionOperator`).
    ///
    /// # Errores
    ///
    /// Retorna un error si no se encuentra un operador válido.
    fn parse_simple_operator(tokens: &mut VecDeque<Token>) -> Result<ExpressionOperator, Errored> {
        let t = tokens
            .front()
            .ok_or_else(|| Syntax("expected operator but was end of query.".to_string()))?;
        let op = match t.value.as_str() {
            "=" => Equals,
            "!=" => NotEquals,
            ">" => GreaterThan,
            ">=" => GreaterOrEqual,
            "<" => LessThan,
            "<=" => LessOrEqual,
            _ => errored!(Syntax, "invalid operator, got: {}", t.value),
        };
        tokens.pop_front();
        Ok(op)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::structs::token::TokenKind::*;
    use crate::query::structs::token::{Token, TokenKind};
    use std::ops::Deref;

    fn create_token(kind: TokenKind, value: &str) -> Token {
        Token {
            kind,
            value: value.to_string(),
        }
    }

    fn operator_should_be(node: &ExpressionNode, op: ExpressionOperator) {
        match node {
            ExpressionNode::Statement { operator, .. } => {
                assert_eq!(*operator, op);
            }
            Empty => {
                if op != None {
                    panic!("Expected a None operator but got {:?}", op)
                }
            }
            _ => panic!("Expected an {:?} expression", op),
        }
    }

    fn leaves_should_have_op(
        node: ExpressionNode,
        l_op: ExpressionOperator,
        r_op: ExpressionOperator,
    ) {
        match node {
            ExpressionNode::Statement { left, right, .. } => {
                operator_should_be(left.deref(), l_op);
                operator_should_be(right.deref(), r_op);
            }
            _ => panic!("Expected an OR expression"),
        }
    }

    #[test]
    fn test_parse_or_expression() {
        let mut tokens = VecDeque::from(vec![
            create_token(Identifier, "x"),
            create_token(Keyword, "="),
            create_token(Number, "1"),
            create_token(Keyword, "OR"),
            create_token(Identifier, "y"),
            create_token(Keyword, "="),
            create_token(Number, "2"),
        ]);

        let result = ExpressionBuilder::parse_expressions(&mut tokens).unwrap();
        operator_should_be(&result, Or);
        leaves_should_have_op(result, Equals, Equals);
    }

    #[test]
    fn test_parse_and_expression() {
        let mut tokens = VecDeque::from(vec![
            create_token(Identifier, "x"),
            create_token(Keyword, "="),
            create_token(Number, "1"),
            create_token(Keyword, "AND"),
            create_token(Identifier, "y"),
            create_token(Keyword, "="),
            create_token(Number, "2"),
        ]);

        let result = ExpressionBuilder::parse_expressions(&mut tokens).unwrap();
        operator_should_be(&result, And);
        leaves_should_have_op(result, Equals, Equals);
    }

    #[test]
    fn test_parse_not_expression() {
        let mut tokens = VecDeque::from(vec![
            create_token(Keyword, "NOT"),
            create_token(ParenthesisOpen, "("),
            create_token(Identifier, "x"),
            create_token(Keyword, "="),
            create_token(Number, "1"),
            create_token(ParenthesisClose, ")"),
        ]);

        let result = ExpressionBuilder::parse_expressions(&mut tokens).unwrap();
        operator_should_be(&result, Not);
        leaves_should_have_op(result, Equals, None)
    }

    #[test]
    fn test_parse_comparison_expression() {
        let mut tokens = VecDeque::from(vec![
            create_token(Identifier, "x"),
            create_token(Keyword, ">"),
            create_token(Number, "10"),
        ]);

        let result = ExpressionBuilder::parse_expressions(&mut tokens).unwrap();
        operator_should_be(&result, GreaterThan)
    }

    #[test]
    fn test_parse_complex_expression() {
        let mut tokens = VecDeque::from(vec![
            create_token(Identifier, "x"),
            create_token(Keyword, "="),
            create_token(Number, "1"),
            create_token(Keyword, "AND"),
            create_token(ParenthesisOpen, "("),
            create_token(Identifier, "y"),
            create_token(Keyword, "="),
            create_token(Number, "2"),
            create_token(Keyword, "OR"),
            create_token(Keyword, "NOT"),
            create_token(Identifier, "z"),
            create_token(Keyword, "="),
            create_token(Number, "3"),
            create_token(ParenthesisClose, ")"),
        ]);

        let result = ExpressionBuilder::parse_expressions(&mut tokens).unwrap();
        operator_should_be(&result, And);
        leaves_should_have_op(result, Equals, Or);
    }

    #[test]
    fn test_parse_invalid_token() {
        let mut tokens = VecDeque::from(vec![
            create_token(Keyword, "INVALID"),
            create_token(Identifier, "x"),
            create_token(Keyword, "="),
            create_token(Number, "1"),
        ]);

        let result = ExpressionBuilder::parse_expressions(&mut tokens);
        assert!(result.is_err());
    }
}
