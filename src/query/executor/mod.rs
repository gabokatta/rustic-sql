use crate::errored;
use crate::query::structs::operation::Operation::*;
use crate::query::structs::query::Query;
use crate::utils::errors::Errored;
use crate::utils::errors::Errored::Syntax;
use crate::utils::files::get_table_path;
use std::path::{Path, PathBuf};

mod delete;
mod insert;
mod select;
mod update;

/// Ejecuta una consulta SQL en una tabla especificada.
///
/// La estructura `Executor` es responsable de llevar a cabo la operación especificada en la consulta
/// SQL sobre la tabla indicada. Esta etapa del procesamiento se lleva acabo luego de haber tokenizado
/// y construido con coherencia sintáctica la consulta.
///
/// Cualquier error que ocurra de ahora en adelante es causado por irregularidades entre la consulta
/// y el archivo destino.
///
/// # Estructura
/// - `table_path`: Ruta del archivo de la tabla sobre la cual se ejecutará la consulta.
/// - `query`: La consulta SQL a ejecutar, representada como un objeto `Query`.
pub struct Executor {
    table_path: PathBuf,
    query: Query,
}

impl Executor {
    /// Crea una nueva instancia de `Executor`.
    ///
    /// # Argumentos
    ///
    /// - `table_path`: La ruta del archivo de la tabla sobre la cual se ejecutará la consulta.
    /// - `query`: La consulta SQL a ejecutar, representada como un objeto `Query`.
    ///
    /// # Retorna
    ///
    /// Una nueva instancia de `Executor`.
    fn new(table_path: PathBuf, query: Query) -> Self {
        Executor { table_path, query }
    }

    /// Ejecuta la consulta SQL especificada.
    ///
    /// Este método determina el tipo de operación (selección, actualización, eliminación, inserción)
    /// basado en la consulta y llama al método correspondiente para realizar la operación.
    ///
    /// # Argumentos
    ///
    /// - `path`: Ruta al directorio donde se encuentran los archivos de las tablas.
    /// - `query`: La consulta SQL a ejecutar.
    ///
    /// # Errores
    ///
    /// Retorna un error si la operación en la consulta no es reconocida o si ocurre un problema al
    /// obtener la ruta de la tabla o al ejecutar la consulta.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    ///
    /// use rustic_sql::query::executor::Executor;
    /// use rustic_sql::query::structs::query::Query;
    /// let query = Query::default();
    /// let result = Executor::run("path/to/tables", query);
    /// match result {
    ///     Ok(()) => println!("Consulta ejecutada exitosamente."),
    ///     Err(e) => eprintln!("Error al ejecutar la consulta: {}", e),
    /// }
    /// ```
    pub fn run(path: &str, query: Query) -> Result<(), Errored> {
        let table_path = get_table_path(Path::new(path), &query.table)?;
        let mut executor = Executor::new(table_path, query);
        match executor.query.operation {
            Select => executor.run_select(),
            Update => executor.run_update(),
            Delete => executor.run_delete(),
            Insert => executor.run_insert(),
            _ => errored!(Syntax, "unknown operation trying to be executed."),
        }
    }
}
