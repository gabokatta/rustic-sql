#[derive(Debug, PartialEq)]
pub enum Operation {
    Unknown,
    Select,
    Update,
    Delete,
    Insert,
}
