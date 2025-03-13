//! The parser for the Oxyl programming language. Returns a "Full" syntax tree.
//! (includes comments and other things).
//!

mod block;
mod comments;
mod expression;
mod ident;
mod statement;
mod utils;
mod value;

pub mod errors;

use crate::fst::File;
use errgonomic::{
    combinators::many,
    parser::{errors::Error, Parser},
};
use errors::ParserError;
use statement::statement;

pub fn parse(input: &str) -> Result<File, Error<&str, ParserError>> {
    many(statement).map(File::new).parse(input)
}
