use crate::query::validate_query_string;
use files::validate_path;
use std::env;
use std::error::Error;

mod errors;
mod files;
mod query;

fn main() -> Result<(), Box<dyn Error>> {
    if let Err(e) = run(env::args().collect()) {
        eprintln!("{}", e);
    }
    Ok(())
}

fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.len() < 3 {
        eprintln!("invalid usage of rustic-sql");
        return Err("usage: cargo run -- <path-to-tables> <sql-query>".into());
    }

    let path: &String = &args[1];
    let query: &String = &args[2];

    validate_path(path)?;
    validate_query_string(query)?;
    dbg!(path, query);

    Ok(())
}
