use crate::query::structs::token::TokenKind::Unknown;

/// Representa un token en una consulta.
///
/// Un token tiene un valor y un tipo (kind) que define su función o significado en la consulta.
///
/// # Ejemplo
///
/// ```rust
/// use rustic_sql::query::structs::token::{Token, TokenKind};
///
/// let token = Token {
///     value: "id_cliente".to_string(),
///     kind: TokenKind::Identifier,
/// };
/// assert_eq!(token.value, "id_cliente");
/// assert_eq!(token.kind, TokenKind::Identifier);
/// ```
#[derive(Debug, PartialEq)]
pub struct Token {
    /// El valor del token como un string.
    pub value: String,

    /// El tipo de token que define su función o significado.
    pub kind: TokenKind,
}

/// Enum que define los posibles tipos de un token.
///
/// Cada variante representa un tipo diferente de token que puede aparecer en una consulta.
///
/// - `Unknown`: Un tipo de token no reconocido.
/// - `String`: Un token que representa una cadena de texto.
/// - `Number`: Un token que representa un número.
/// - `Operator`: Un token que representa un operador.
/// - `Identifier`: Un token que representa un identificador o variable.
/// - `ParenthesisOpen`: Un token que representa un paréntesis de apertura.
/// - `ParenthesisClose`: Un token que representa un paréntesis de cierre.
/// - `Keyword`: Un token que representa una palabra clave de SQL.
///
/// # Ejemplo
///
/// ```rust
/// use rustic_sql::query::structs::token::TokenKind;
///
/// let kind = TokenKind::String;
/// assert_eq!(kind, TokenKind::String);
/// ```
#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Unknown,
    String,
    Number,
    Operator,
    Identifier,
    ParenthesisOpen,
    ParenthesisClose,
    Keyword,
}

impl Default for Token {
    /// Crea un token con valores predeterminados.
    fn default() -> Self {
        Self {
            value: String::new(),
            kind: Unknown,
        }
    }
}
