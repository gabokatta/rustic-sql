use crate::errored;
use crate::query::structs::comparator::ExpressionComparator;
use crate::query::structs::expression::ExpressionResult::{Bool, Int, Str};
use crate::query::structs::token::{Token, TokenKind};
use crate::utils::errors::Errored;
use crate::utils::errors::Errored::{Column, Default, Syntax};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

/// Enum que representa a una expresión.
///
/// Usando una estructura recursiva de nodos, el mismo puede ser un nodo vacío, una hoja
/// con un token, o una declaración con un operador y dos sub-nodos (izquierdo y derecho).
#[derive(Default, PartialEq)]
pub enum ExpressionNode {
    #[default]
    Empty,
    Leaf(Token),
    Statement {
        operator: ExpressionOperator,
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
    },
}

/// Enum que define los operadores posibles en una expresión.
///
/// Los operadores incluyen comparación (igual, mayor.. etc.) y operadores
/// lógicos (AND, OR, NOT).
#[derive(Debug, Default, PartialEq)]
pub enum ExpressionOperator {
    #[default]
    None,
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
    And,
    Or,
    Not,
}

/// Enum que representa los posibles resultados de una expresión.
///
/// Los resultados pueden ser un entero, un string  o un valor booleano.
#[derive(Debug, PartialEq)]
pub enum ExpressionResult {
    Int(i64),
    Str(String),
    Bool(bool),
}

impl ExpressionNode {
    /// Evalúa el nodo de expresión usando los valores proporcionados.
    /// Dichos valores estan contenidos dentro de un mapa que representa el contexto actual
    /// de la ejecución.
    ///
    /// # Parámetros
    ///
    /// * `values` - Un `HashMap` que contiene los pares clave, valor del contexto actual.
    ///
    /// # Retorna
    ///
    /// Un `Result` que contiene el resultado de la evaluación de la expresión o un error en caso de
    /// que ocurra algún problema.
    pub fn evaluate(&self, values: &HashMap<String, String>) -> Result<ExpressionResult, Errored> {
        match self {
            ExpressionNode::Empty => Ok(Bool(true)),
            ExpressionNode::Leaf(t) => match t.kind {
                TokenKind::Identifier => ExpressionNode::get_variable_value(values, t),
                TokenKind::String => Ok(Str(t.value.to_string())),
                TokenKind::Number => Ok(Int(t.value.parse::<i64>()?)),
                _ => Ok(Bool(false)),
            },
            ExpressionNode::Statement {
                operator,
                left,
                right,
            } => {
                let l = left.evaluate(values)?;
                let r = right.evaluate(values)?;
                ExpressionNode::get_statement_value(operator, l, r)
            }
        }
    }

    /// Obtiene el valor de una declaración comparativa.
    /// El método comparativo a ser ejecutado depende de los tipos de datos contenidos
    /// en las hojas de la expresión.
    ///
    /// # Parámetros
    ///
    /// * `operator` - El operador de la expresión.
    /// * `left` - El resultado de la evaluación del lado izquierdo de la expresión.
    /// * `right` - El resultado de la evaluación del lado derecho de la expresión.
    ///
    /// # Retorna
    ///
    /// Un `Result` que contiene el resultado de la comparación o un error en caso de que los tipos
    /// no coincidan.
    fn get_statement_value(
        operator: &ExpressionOperator,
        left: ExpressionResult,
        right: ExpressionResult,
    ) -> Result<ExpressionResult, Errored> {
        match (left, right) {
            (Int(l), Int(r)) => ExpressionComparator::compare_ints(l, r, operator),
            (Str(l), Str(r)) => ExpressionComparator::compare_str(&l, &r, operator),
            (Bool(l), Bool(r)) => ExpressionComparator::compare_bools(l, r, operator),
            _ => errored!(Syntax, "expression members must match in type."),
        }
    }

    /// Obtiene el valor de una variable a partir del `HashMap` de valores.
    /// Dicho `HashMap`vendría a ser el contexto en donde se esta interprentando la
    /// expresión.
    ///
    /// # Parámetros
    ///
    /// * `values` - Un `HashMap` que contiene los pares clave, valor del contexto actual.
    /// * `t` - El token que representa la variable.
    ///
    /// # Retorna
    ///
    /// Un `Result` que contiene el valor de la variable o un error si la variable no existe.
    pub fn get_variable_value(
        values: &HashMap<String, String>,
        t: &Token,
    ) -> Result<ExpressionResult, Errored> {
        let value = values.get(&t.value);
        match value {
            Some(v) => {
                if v.parse::<i64>().is_ok() {
                    Ok(Int(v.parse::<i64>()?))
                } else {
                    Ok(Str(v.to_string()))
                }
            }
            None => errored!(Column, "column {} does not exist", t.value),
        }
    }

    /// Obtiene una tupla de los tokens de una declaración que son hojas.
    /// Este método es usado para representar las actualizaciones de una consulta.
    /// Ya que una actualización tiene una llave y un valor, nos conviene devolver en un par.
    ///
    /// # Retorna
    ///
    /// Un `Result` que contiene una tupla con los dos tokens de las hojas o un error si los nodos
    /// no son hojas.
    pub fn as_leaf_tuple(&self) -> Result<(&Token, &Token), Errored> {
        match self {
            ExpressionNode::Statement { left, right, .. } => match (&**left, &**right) {
                (ExpressionNode::Leaf(l), ExpressionNode::Leaf(r)) => Ok((l, r)),
                _ => errored!(Default, "both sides of expression must be leaf nodes."),
            },
            _ => errored!(Default, "expected a statement, but got: {:?}", self),
        }
    }
}

impl Debug for ExpressionNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionNode::Empty => write!(f, "()"),
            ExpressionNode::Leaf(t) => write!(f, "{}", t.value),
            ExpressionNode::Statement {
                operator,
                left,
                right,
            } => {
                write!(f, "{:?}[{:?},{:?}]", operator, left, right)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::structs::token::Token;
    use crate::query::structs::token::TokenKind::*;
    use std::collections::HashMap;

    #[test]
    fn test_evaluate_empty_node() {
        let node = ExpressionNode::Empty;
        assert_eq!(node.evaluate(&HashMap::new()).unwrap(), Bool(true));
    }

    #[test]
    fn test_evaluate_leaf_identifier() {
        let mut values = HashMap::new();
        values.insert("id_cliente".to_string(), "123".to_string());
        let node = ExpressionNode::Leaf(Token {
            kind: Identifier,
            value: "id_cliente".to_string(),
        });
        assert_eq!(node.evaluate(&values).unwrap(), Int(123));
    }

    #[test]
    fn test_evaluate_leaf_string() {
        let node = ExpressionNode::Leaf(Token {
            kind: String,
            value: "buenaaaaas".to_string(),
        });
        assert_eq!(
            node.evaluate(&HashMap::new()).unwrap(),
            Str("buenaaaaas".to_string())
        );
    }

    #[test]
    fn test_evaluate_leaf_number() {
        let node = ExpressionNode::Leaf(Token {
            kind: Number,
            value: "360".to_string(),
        });
        assert_eq!(node.evaluate(&HashMap::new()).unwrap(), Int(360));
    }

    #[test]
    fn test_evaluate_invalid_token() {
        let node = ExpressionNode::Leaf(Token {
            kind: Keyword,
            value: "".to_string(),
        });
        assert_eq!(node.evaluate(&HashMap::new()).unwrap(), Bool(false));
    }

    #[test]
    fn test_evaluate_statement_equal_int() {
        let left = ExpressionNode::Leaf(Token {
            kind: Number,
            value: "360".to_string(),
        });
        let right = ExpressionNode::Leaf(Token {
            kind: Number,
            value: "360".to_string(),
        });
        let node = ExpressionNode::Statement {
            operator: ExpressionOperator::Equals,
            left: Box::new(left),
            right: Box::new(right),
        };
        assert_eq!(node.evaluate(&HashMap::new()).unwrap(), Bool(true));
    }

    #[test]
    fn test_evaluate_statement_not_equal_str() {
        let left = ExpressionNode::Leaf(Token {
            kind: String,
            value: "rust".to_string(),
        });
        let right = ExpressionNode::Leaf(Token {
            kind: String,
            value: "gleam".to_string(),
        });
        let node = ExpressionNode::Statement {
            operator: ExpressionOperator::NotEquals,
            left: Box::new(left),
            right: Box::new(right),
        };
        assert_eq!(node.evaluate(&HashMap::new()).unwrap(), Bool(true));
    }

    #[test]
    fn test_get_variable_value_existing() {
        let mut values = HashMap::new();
        values.insert("id_cliente".to_string(), "789".to_string());
        let token = Token {
            kind: Identifier,
            value: "id_cliente".to_string(),
        };
        assert_eq!(
            ExpressionNode::get_variable_value(&values, &token).unwrap(),
            Int(789)
        );
    }

    #[test]
    fn test_get_variable_value_non_existing() {
        let values = HashMap::new();
        let token = Token {
            kind: Identifier,
            value: "id".to_string(),
        };
        assert!(ExpressionNode::get_variable_value(&values, &token).is_err());
    }

    #[test]
    fn test_as_leaf_tuple_valid() {
        let left = ExpressionNode::Leaf(Token {
            kind: Identifier,
            value: "id_cliente".to_string(),
        });
        let right = ExpressionNode::Leaf(Token {
            kind: Identifier,
            value: "360".to_string(),
        });
        let node = ExpressionNode::Statement {
            operator: ExpressionOperator::Equals,
            left: Box::new(left),
            right: Box::new(right),
        };
        let (l, r) = node.as_leaf_tuple().unwrap();
        assert_eq!(l.value, "id_cliente");
        assert_eq!(r.value, "360");
    }

    #[test]
    fn test_as_leaf_tuple_invalid() {
        let left = ExpressionNode::Statement {
            operator: ExpressionOperator::Equals,
            left: Box::new(ExpressionNode::Empty),
            right: Box::new(ExpressionNode::Empty),
        };
        let right = ExpressionNode::Leaf(Token {
            kind: Identifier,
            value: "col1".to_string(),
        });
        let node = ExpressionNode::Statement {
            operator: ExpressionOperator::Equals,
            left: Box::new(left),
            right: Box::new(right),
        };
        assert!(node.as_leaf_tuple().is_err());
    }
}
