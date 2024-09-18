use crate::query::structs::ordering::OrderKind::Asc;
use crate::query::structs::token::Token;
use std::fmt::{Debug, Formatter};

/// Estructura que representa un criterio de ordenamiento dentro de RusticSQL.
///
/// Esta estructura se utiliza para definir cómo se deben ordenar los resultados en una consulta.
///
/// # Campos
///
/// * `field` - El token que representa el campo por el cual se realizará el ordenamiento.
/// * `kind` - El tipo de ordenamiento (ascendente o descendente).
#[derive(PartialEq)]
pub struct Ordering {
    pub field: Token,
    pub kind: OrderKind,
}

/// Enum que representa los tipos de ordenamiento posibles.
///
/// Los tipos de ordenamiento incluyen:
///
/// - `Asc`: Ordena los resultados de manera ascendente.
/// - `Desc`: Ordena los resultados de manera descendente.
#[derive(Debug, PartialEq)]
pub enum OrderKind {
    Asc,
    Desc,
}

impl Default for Ordering {
    /// Devuelve un valor default para `Ordering`.
    ///
    /// El valor default utiliza el token predeterminado y el tipo de ordenamiento ascendente.
    fn default() -> Self {
        Self {
            field: Token::default(),
            kind: Asc,
        }
    }
}

impl Debug for Ordering {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{:?})", &self.field.value, &self.kind)
    }
}
