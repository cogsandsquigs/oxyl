use errgonomic::parser::errors::CustomError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ParserError {}

impl CustomError for ParserError {}
