use crate::query::executor::Executor;
use crate::query::tokenizer::Tokenizer;
use crate::query::Query;
use query::validate_query_string;
use std::env;
use std::error::Error;

mod query;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect();
    if let Err(e) = run(args) {
        println!("{}", e);
    }
    Ok(())
}

fn run(args: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
    if args.len() != 3 {
        println!("invalid usage of rustic-sql");
        return Err("usage: cargo run -- <path-to-tables> <sql-query>".into());
    }

    let path: &String = &args[1];
    let query: &String = &args[2];
    validate_query_string(query)?;

    let tokens = Tokenizer::new().tokenize(query)?;
    let query = Query::from(tokens)?;
    println!("\n{:?}", &query);
    let result = Executor::run(path, query)?;
    Ok(result)
}
