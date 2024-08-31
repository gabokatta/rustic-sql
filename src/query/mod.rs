mod errors;
mod executor;
mod parser;

use crate::errors::Errored;

pub fn validate_query_string(query: &str) -> Result<(), Errored> {
    if query.trim().is_empty() {
        return Err(Errored(String::from("query is empty.")));
    }
    Ok(())
}
