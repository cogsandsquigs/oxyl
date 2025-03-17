use super::super::{
    block::block,
    errors::ParserError,
    utils::{parenthesized, wnnw},
    value::value,
};
use crate::{
    compile::parser::utils::ww,
    repr::fst::{
        application::Application,
        expression::{Expression, ExpressionKind, Operator, OperatorKind},
        FstNode,
    },
};
use errgonomic::{
    combinators::any,
    parser::{errors::Result, state::State, Parser},
    prelude::{is, Associativity, Pratt},
};

/// Parses an application of a function.
/// ```bnf
/// <application> ::= <ident> <expression>
/// ```
pub fn application(state: State<&str, ParserError>) -> Result<&str, Application, ParserError> {
    todo!()
}
