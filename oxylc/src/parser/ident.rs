use super::errors::ParserError;
use crate::ast::identifier::Identifier;
use errgonomic::{
    combinators::{alphabetic, alphanumeric, any, is, many},
    parser::{errors::Result, state::State, Parser},
};

/// Parses an ident.
/// ```bnf
/// <ident> ::= [a-zA-Z] ( [a-zA-Z0-9] | "_" )*
/// ```
pub fn ident(state: State<&str, ParserError>) -> Result<&str, Identifier, ParserError> {
    alphabetic
        .then(many(any((alphanumeric, is("_")))))
        .map(|(part1, part2s)| {
            let part2 = part2s.into_iter().reduce(|acc, x| acc.join(&x));
            let ident = match part2 {
                Some(x) => part1.join(&x),
                None => part1,
            };

            Identifier::new(ident.as_inner().to_string(), ident.span())
        })
        .process(state)
}
