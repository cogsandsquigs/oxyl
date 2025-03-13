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
    many(statement)
        .map_with_state(|state, stmts| {
            let location = state.as_input().span().union_between((0..1).into());
            (state, File::new(location, stmts))
        })
        .parse(input)
}
