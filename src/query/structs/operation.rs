/// Enum que representa las diferentes operaciones que se pueden realizar dentro de RusticSQL.
///
/// Las operaciones incluyen:
///
/// - `Unknown`: La operación es desconocida o no se ha especificado.
/// - `Select`: Selecciona datos de una tabla.
/// - `Update`: Actualiza datos existentes en una tabla.
/// - `Delete`: Elimina datos de una tabla.
/// - `Insert`: Inserta nuevos datos en una tabla.
#[derive(Debug, PartialEq)]
pub enum Operation {
    Unknown,
    Select,
    Update,
    Delete,
    Insert,
}
