use crate::ast::Ast;
use errgonomic::{
    combinators::many,
    parser::{errors::Error, Parser},
};
use errors::ParserError;
use statement::statement;

mod expression;
mod ident;
mod statement;
mod utils;
mod value;

pub mod errors;

/// Parses the AST representing a singular file.
/// ```bnf
/// <oxyl> ::= <statement>*
/// ```
pub fn parser(input: &str) -> Result<Ast, Error<&str, ParserError>> {
    many(statement).map(Ast::new).parse(input)
}
