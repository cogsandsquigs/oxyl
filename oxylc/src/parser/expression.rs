use super::{errors::ParserError, value::value};
use crate::ast::{
    expression::{Expression, ExpressionKind},
    AstNode,
};
use errgonomic::{
    combinators::{any, between, is, whitespace_wrapped as ww},
    parser::{errors::Result, state::State, Parser},
};

/// Parses an `Expression` object.
/// ```bnf
/// <expression> ::= ("(" <expression> ")") | <value>
/// ```
pub fn expression(state: State<&str, ParserError>) -> Result<&str, Expression, ParserError> {
    any((
        ww(expression),                        // Expression wrapped in whitespace
        between(is("("), expression, is(")")), // Paren expression
        // Actual "expressions" start here
        value.map(|value| {
            let location = *value.location();
            Expression::new(ExpressionKind::Value(value), location)
        }),
    ))
    .process(state)
}
