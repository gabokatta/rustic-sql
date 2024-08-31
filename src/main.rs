use files::validate_path;
use query::validate_query_string;
use std::env;
use std::error::Error;

mod errors;
mod files;
mod query;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect();
    if let Err(e) = run(args) {
        println!("{}", e);
    }
    Ok(())
}

fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.len() < 3 {
        println!("invalid usage of rustic-sql");
        return Err("usage: cargo run -- <path-to-tables> <sql-query>".into());
    }

    let path: &String = &args[1];
    let query: &String = &args[2];

    validate_path(path)?;
    validate_query_string(query)?;
    dbg!(path, query);

    Ok(())
}
