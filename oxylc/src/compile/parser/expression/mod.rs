mod tests;
mod utils;

use super::{
    block::block,
    errors::ParserError,
    utils::{parenthesized, wnnw},
    value::value,
};
use crate::{
    compile::parser::utils::ww,
    fst::{
        expression::{Expression, ExpressionKind, Operator, OperatorKind},
        FstNode,
    },
};
use errgonomic::{
    combinators::any,
    parser::{errors::Result, state::State, Parser},
    prelude::{is, Associativity, Pratt},
};
use utils::*;

/// Parses an `Expression` object.
/// ```bnf
/// <expression> ::= atom
/// ```
pub fn expression(state: State<&str, ParserError>) -> Result<&str, Expression, ParserError> {
    pratt.process(state)
}

/// Parses an atomic `Expression` object.
/// ```bnf
/// <atom> ::= ( "(" <expression> ")" ) | <value>
/// ```
pub fn atom(state: State<&str, ParserError>) -> Result<&str, Expression, ParserError> {
    // NOTE: Don't do `ww(expression)` in the `any`, as we simply recurse forever if we never
    // encounter an expression. Therefore, `ww` every individual kind of expression
    any((
        wnnw(value.map(|value| Expression::new(*value.location(), ExpressionKind::Value(value)))),
        wnnw(block.map(|block| Expression::new(*block.location(), ExpressionKind::Block(block)))),
        wnnw(parenthesized(expression)).map(|(p1, expr, p2)| {
            Expression::new(
                p1.span().union_between(p2.span()),
                ExpressionKind::Parenthesized {
                    lparen_location: p1.span(),
                    rparen_location: p2.span(),
                    inner: Box::new(expr),
                },
            )
        }), // Paren expression
    ))
    .process(state)
}

/// The pratt parser we are using.
/// TODO: Make this not re-create pratt parser on every call?
fn pratt(state: State<&str, ParserError>) -> Result<&str, Expression, ParserError> {
    Pratt::new(atom, cons_prefix, cons_infix, cons_postfix)
        .with_prefix_op(ww(is("-")).map(|op| Operator::new(op.span(), OperatorKind::Dash)))
        // NOTE: We want `.` on top as it "binds tighter" than `|>`, so out of an expression
        // `a.b |> c` we get `(a.b) |> c`.
        .with_infix_op(
            ww(is(".")).map(|op| Operator::new(op.span(), OperatorKind::Dot)),
            Associativity::Right,
        )
        .with_infix_op(
            ww(is("|>")).map(|op| Operator::new(op.span(), OperatorKind::Triangle)),
            Associativity::Right,
        )
        .with_infix_op(
            ww(is("*")).map(|op| Operator::new(op.span(), OperatorKind::Star)),
            Associativity::Left,
        )
        .with_infix_op(
            ww(is("/")).map(|op| Operator::new(op.span(), OperatorKind::FSlash)),
            Associativity::Left,
        )
        .with_infix_op(
            ww(is("+")).map(|op| Operator::new(op.span(), OperatorKind::Plus)),
            Associativity::Left,
        )
        .with_infix_op(
            ww(is("-")).map(|op| Operator::new(op.span(), OperatorKind::Dash)),
            Associativity::Left,
        )
        .process(state)
}
