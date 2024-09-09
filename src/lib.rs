use crate::query::executor::Executor;
use crate::query::structs::query::Query;
use crate::query::tokenizer::Tokenizer;
use crate::query::validate_query_string;
use crate::utils::files::validate_path;
use std::error::Error;

pub mod query;
pub mod utils;

/// Ejecuta la aplicación RusticSQL a partir de los argumentos proporcionados.
///
/// Esta función valida los argumentos de la línea de comandos, procesa la consulta SQL y ejecuta
/// la consulta en los datos especificados.
///
/// # Argumentos
///
/// - `args`: Un vector de `String` que contiene los argumentos de la línea de comandos. Se espera que
///   contenga exactamente tres elementos: el nombre del comando, la ruta a las tablas y la consulta SQL.
///
/// # Errores
///
/// - Retorna un error si el número de argumentos es incorrecto.
/// - Retorna un error si la ruta a las tablas no es válida.
/// - Retorna un error si la consulta SQL no es válida.
/// - Retorna un error si el procesamiento de la consulta o la ejecución falla.
pub fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.len() != 3 {
        println!("invalid usage of rustic-sql");
        return Err("usage: cargo run -- <path-to-tables> <sql-query>".into());
    }

    let path: &String = &args[1];
    let query: &String = &args[2];
    validate_path(path)?;
    validate_query_string(query)?;

    let tokens = Tokenizer::new().tokenize(query)?;
    let query = Query::from(tokens)?;
    Executor::run(path, query)?;
    Ok(())
}
