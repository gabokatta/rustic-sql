//! # RusticSQL 🦀
//!
//! ### Clon de SQL, en Rust.
//!
//! Permite al usuario ejecutar consultas SQL mediante la linea de comandos.
//!
//! Las operaciones se realizan sobre "tablas" (archivos csv).
//!
//!
//! Consultas Permitidas: [SELECT, INSERT, UPDATE, DELETE]
//!
//! Operadores Disponibles: [AND, OR, NOT y comparadores simples (>, <, =, etc..)]
//!
//!
//! Estructura del Proyecto:
//! - Tokenizador: Recibe un String y te devuelve tokens.
//! - Constructor: Recibe tokens y los transforma en consultas validas.
//! - Ejecutor: Recibe consultas y las ejecuta sobre las tablas.
//!
//! # Usar RusticSQL:
//!
//! > ```BASH
//! > cargo run -- ruta/a/tablas "SELECT * FROM table" > output.csv
//! > ```
//!
//! # Testea RusticSQL:
//!
//! >```BASH
//! >cargo test --all
//! >```
//!
//!
#![doc(
    html_logo_url = "https://cdn-icons-png.flaticon.com/512/4726/4726022.png",
    html_favicon_url = "https://cdn-icons-png.flaticon.com/512/4726/4726022.png"
)]

use crate::query::executor::Executor;
use crate::query::structs::query::Query;
use crate::query::tokenizer::Tokenizer;
use crate::query::validate_query_string;
use crate::utils::files::validate_path;
use std::error::Error;

pub mod query;
pub mod utils;

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
