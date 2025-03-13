use super::errors::ParserError;
use errgonomic::{
    combinators::{
        any, between, commit, consumed, eoi, is, newlines, take_until, whitespace,
        whitespace_not_newline,
    },
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

/*
/// Shorthand for our modified `whitespace_wrapped`, but includes comments
pub fn ww<'a, O, P: Parser<&'a str, O, ParserError>>(p: P) -> impl Parser<&'a str, O, ParserError> {
    between(any((comment, whitespace)), p, any((comment, whitespace)))
}

/// Shorthand for our modified `whitespace_not_newline_wrapped`, but includes comments
pub fn wnnw<'a, O, P: Parser<&'a str, O, ParserError>>(
    p: P,
) -> impl Parser<&'a str, O, ParserError> {
    between(
        any((comment, whitespace_not_newline)),
        p,
        any((comment, whitespace_not_newline)),
    )
}
*/
