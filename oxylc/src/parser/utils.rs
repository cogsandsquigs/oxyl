use super::errors::ParserError;
use errgonomic::{
    combinators::{any, commit, consumed, eoi, is, newlines},
    parser::{errors::Result, input::Input, state::State, Parser},
};

/// A line ending
/// ```bnf
/// <line_ending> ::= ( NEWLINE | EOI )
/// ```
pub fn line_ending(state: State<&str, ParserError>) -> Result<&str, Input<&str>, ParserError> {
    any((newlines, consumed(eoi))).process(state)
}

/// A parenthesized expression
/// ```bnf
/// <parenthesized> ::= "(" your_expression_here ")"
/// ```
pub fn parenthesized<'a, O, P: Parser<&'a str, O, ParserError>>(
    p: P,
) -> impl Parser<&'a str, O, ParserError> {
    is("(").then(commit(p.then(is(")")))).map(|(_, (o, _))| o)
}
