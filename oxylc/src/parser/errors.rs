use std::num::{ParseFloatError, ParseIntError};

use errgonomic::parser::errors::CustomError;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParserError {
    #[error("Error parsing integer: ")]
    ParseInt(#[from] ParseIntError),

    #[error("Error parsing floating number: ")]
    ParseFloat(#[from] ParseFloatError),
}

impl CustomError for ParserError {}
