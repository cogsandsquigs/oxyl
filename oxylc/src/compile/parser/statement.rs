use super::{
    errors::ParserError,
    expression::expression,
    ident::ident,
    utils::{line_ending, wnnw, ww},
};
use crate::repr::fst::statement::{Statement, StatementKind};
use errgonomic::{
    combinators::{any, commit, is},
    parser::{errors::Result, state::State, Parser},
};

/// Parses a `Statement` object.
/// ```bnf
/// <statement> ::= <let_stmt>
/// ```
pub fn statement(state: State<&str, ParserError>) -> Result<&str, Statement, ParserError> {
    // NOTE: Don't do `ww(statement)` in the `any`, as we simply recurse forever if we never
    // encounter a statement. Therefore, `ww` every individual kind of statement.
    any((ww(let_stmt),)).process(state)
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
            wnnw(ident)
                .then(ww(is("=")))
                .then(expression) // NOTE: alr. wrapped in whitespace
                .then(line_ending),
        ))
        .map_with_state(|state, (let_kwd, (((ident, _), expression), ending))| {
            let location = let_kwd.span().union_between(ending.span());
            (
                state,
                Statement::new(
                    location,
                    StatementKind::Let {
                        is_mutable: false, // TODO: Mutability in the future?
                        ident,
                        expression,
                    },
                ),
            )
        })
        .process(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repr::fst::{
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
                ident: Identifier::new((4..7).into(), "abc".into(),),
                expression: Expression::new(
                    (10..13).into(),
                    ExpressionKind::Value(Value::new((10..13).into(), ValueKind::Integer(123),)),
                )
            }
        );
        assert_eq!(state.as_input().as_inner(), "");
    }
}
