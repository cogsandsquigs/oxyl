use std::num::ParseIntError;

use errgonomic::parser::errors::CustomError;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParserError {
    #[error("Error parsing integer: ")]
    ParseIntError(#[from] ParseIntError),
}

impl CustomError for ParserError {}
