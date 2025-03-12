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
        let_stmt,
        // NOTE: This must come last, because otherwise we recurse forever
        ww(statement), // Statement wrapped in whitespace
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{
        expression::{Expression, ExpressionKind},
        identifier::Identifier,
        value::{Value, ValueKind},
    };

    #[test]
    fn can_parse_let_statement() {
        let (state, stmt) = statement.process("let abc = 123\n".into()).unwrap();
        assert_eq!(
            stmt.kind(),
            &StatementKind::Let {
                is_mutable: false,
                ident: Identifier::new("abc".into(), (4..7).into()),
                expression: Expression::new(
                    ExpressionKind::Value(Value::new(ValueKind::Integer(123), (10..13).into())),
                    (10..13).into()
                )
            }
        );
        assert_eq!(state.as_input().as_inner(), "");
    }
}
