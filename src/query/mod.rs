pub mod builder;
mod errors;
pub mod executor;
pub mod structs;
pub mod tokenizer;

use crate::errored;
use crate::utils::errors::Errored;

pub fn validate_query_string(query: &str) -> Result<(), Errored> {
    if query.trim().is_empty() {
        errored!(Errored, "query is empty.");
    }
    Ok(())
}
