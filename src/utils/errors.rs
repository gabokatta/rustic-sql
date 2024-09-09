use crate::utils::errors::Errored::*;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::num::ParseIntError;

/// Macro para crear errores personalizados con los errores `Errored`.
///
/// Este macro facilita la creación de errores personalizados al construir un `Errored`
/// con un mensaje formateado, una vez creado el error la misma macro lo retorna.
///
/// # Ejemplo
///
/// ```rust
///
/// use rustic_sql::errored;
/// use rustic_sql::utils::errors::Errored;
/// use rustic_sql::utils::errors::Errored::Syntax;
///
/// fn example_function() -> Result<(), Errored> {
///     let some_condition = true;
///     if some_condition {
///         errored!(Syntax, "An error occurred with syntax");
///     }
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! errored {
    ($err_type:ident, $msg:expr) => {
        return Err($err_type(format!($msg)))
    };
    ($err_type:ident, $fmt:expr, $($arg:tt)*) => {
        return Err($err_type(format!($fmt, $($arg)*)))
    };
}

/// Enum que representa diferentes tipos de errores en la aplicación RusticSQL.
///
/// Este enum permite clasificar los errores que pueden ocurrir durante el procesamiento de
/// datos o la ejecución de operaciones, proporcionando mensajes descriptivos para cada tipo de error.
///
/// # Variantes
///
/// - `Syntax(String)`: Representa un error relacionado con la sintaxis.
/// - `Column(String)`: Representa un error relacionado con una columna.
/// - `Table(String)`: Representa un error relacionado con una tabla.
/// - `Default(String)`: Representa un error genérico.
///
pub enum Errored {
    Syntax(String),
    Column(String),
    Table(String),
    Default(String),
}

impl Error for Errored {}

impl Debug for Errored {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Errored {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Syntax(syntax) => {
                write!(f, "[INVALID_SYNTAX]: {}", syntax)
            }
            Column(column) => {
                write!(f, "[INVALID_COLUMN]: {}", column)
            }
            Table(table) => {
                write!(f, "[INVALID_TABLE]: {}", table)
            }
            Default(error) => {
                write!(f, "[ERROR]: {}", error)
            }
        }
    }
}

impl From<io::Error> for Errored {
    fn from(value: io::Error) -> Self {
        Default(format!("IO - {}", value))
    }
}

impl From<ParseIntError> for Errored {
    fn from(value: ParseIntError) -> Self {
        Default(format!("PARSE_INT - {}", value))
    }
}
