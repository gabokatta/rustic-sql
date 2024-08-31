use files::validate_path;
use std::env;
use std::error::Error;
mod errors;
mod files;
mod query;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("invalid usage of rustic-sql");
        return Err("usage: cargo run -- <path-to-tables> <sql-query>".into());
    }

    let path: &String = &args[1];
    let query: &String = &args[2];

    validate_path(path)?;
    dbg!(path, query);

    Ok(())
}
