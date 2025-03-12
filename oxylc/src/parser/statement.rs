use super::{errors::ParserError, expression::expression, ident::ident, utils::line_ending};
use crate::ast::statement::{Statement, StatementKind};
use errgonomic::{
    combinators::{any, commit, is, maybe, whitespace_wrapped as ww},
    parser::{errors::Result, state::State, Parser},
};

/// Parses a `Statement` object.
/// ```bnf
/// <statement> ::= <let_stmt>
/// ```
pub fn statement(state: State<&str, ParserError>) -> Result<&str, Statement, ParserError> {
    any((
        ww(statement), // Statement wrapped in whitespace
        // Actual "statements" start here
        let_stmt,
    ))
    .process(state)
}

/// A `let`-statement.
/// ```bnf
/// <let_stmt> ::= "let" <ident> "=" <expression> <line_ending>
/// ```
fn let_stmt(state: State<&str, ParserError>) -> Result<&str, Statement, ParserError> {
    is("let")
        // NOTE: commit on the rest of the statement, as we know we must parse a `let` statement
        // now.
        .then(commit(
            maybe(ww(is("mut")))
                .then(ww(ident))
                .then(is("="))
                .then(expression) // NOTE: alr. wrapped in whitespace
                .then(line_ending),
        ))
        .map_with_state(
            |state, (let_kwd, ((((is_mut, ident), _), expression), _))| {
                let location = let_kwd.span().union_between(state.as_input().span());
                (
                    state,
                    Statement::new(
                        StatementKind::Let {
                            is_mutable: is_mut.is_some(),
                            ident,
                            expression,
                        },
                        location,
                    ),
                )
            },
        )
        .process(state)
}
