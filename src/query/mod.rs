pub mod builder;
pub mod executor;
pub mod structs;
pub mod tokenizer;

use crate::errored;
use crate::utils::errors::Errored;
use crate::utils::errors::Errored::Default;

pub fn validate_query_string(query: &str) -> Result<(), Errored> {
    if query.trim().is_empty() {
        errored!(Default, "query is empty.");
    }
    Ok(())
}
