use std::num::{ParseFloatError, ParseIntError};

use errgonomic::parser::errors::CustomError;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParserError {
    #[error("Error parsing integer: {0}")]
    ParseInt(#[from] ParseIntError),

    #[error("Error parsing floating number: {0}")]
    ParseFloat(#[from] ParseFloatError),
}

impl CustomError for ParserError {}
