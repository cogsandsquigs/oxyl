use super::atom;
use crate::{
    compile::parser::{errors::ParserError, utils::wnnw},
    repr::fst::{
        expression::{Expression, ExpressionKind},
        FstNode,
    },
};
use errgonomic::{
    parser::Parser,
    prelude::{Result, State},
};

/// Takes an atom, and tries to apply it to the next one.
/// TODO: Eugh, not an actual parser. Somehow fix that at some point?
/// WARN: This should be parsed last, since otherwise it'll just recurse forever!
pub fn application(
    state: State<&str, ParserError>,
    left: Expression,
) -> Result<&str, Expression, ParserError> {
    let (state, right) = wnnw(atom).process(state)?;
    Ok((state, fix_application(left, right)))
}

/// NOTE: We assume that we are to create an application between `left` and `right`.
/// In an ideal world, we have `((h g) f) a)` from `h g f a`, and to enforce this we use an
/// inductive approach:
///     Base case: <expr> <expr>
///         1. just make `appl { <expr> <expr> }`
///     Inductive case: <expr> <appl>, asm. <appl> is "correct" (e.g. `((g f) a)` from `g f a`), so
///     like `h g f a`:
///         1. unwrap into `h, g, (f ...)`
fn fix_application(left: Expression, right: Expression) -> Expression {
    match right.kind {
        ExpressionKind::Application { function, arg } => fix_application(
            Expression::new(
                left.location().union_between(*function.location()),
                ExpressionKind::Application {
                    function: Box::new(left),
                    arg: function.clone(), // TODO: Less expensive please!
                },
            ),
            *arg,
        ),
        _ => Expression::new(
            left.location().union_between(*right.location()),
            ExpressionKind::Application {
                function: Box::new(left),
                arg: Box::new(right),
            },
        ),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        compile::parser::expression::expression,
        repr::fst::{
            identifier::Identifier,
            value::{Value, ValueKind},
        },
    };

    use super::*;

    #[test]
    fn can_parse_simple_app() {
        let (state, parsed) = expression.process("f g".into()).unwrap();
        assert!(state.is_ok());
        assert_eq!(state.as_input().as_inner(), "");
        assert_eq!(
            parsed,
            Expression::new(
                (0..3).into(),
                ExpressionKind::Application {
                    function: Box::new(Expression::new(
                        (0..1).into(),
                        ExpressionKind::Value(Value::new(
                            (0..1).into(),
                            ValueKind::Identifier(Identifier::new((0..1).into(), "f".to_string())),
                        )),
                    )),
                    arg: Box::new(Expression::new(
                        (2..3).into(),
                        ExpressionKind::Value(Value::new(
                            (2..3).into(),
                            ValueKind::Identifier(Identifier::new((2..3).into(), "g".to_string())),
                        )),
                    )),
                },
            ),
        )
    }

    #[test]
    // NOTE: This requires that `application` is in invoked within `atom`, as it requires recursive
    // calls to `atom`.
    fn can_parse_app_2_args() {
        let (state, parsed) = atom.process("f g h".into()).unwrap();
        assert!(state.is_ok());
        assert_eq!(state.as_input().as_inner(), "");
        assert_eq!(
            parsed,
            Expression::new(
                (0..5).into(),
                ExpressionKind::Application {
                    function: Box::new(Expression::new(
                        (0..3).into(),
                        ExpressionKind::Application {
                            function: Box::new(Expression::new(
                                (0..1).into(),
                                ExpressionKind::Value(Value::new(
                                    (0..1).into(),
                                    ValueKind::Identifier(Identifier::new(
                                        (0..1).into(),
                                        "f".to_string()
                                    )),
                                )),
                            )),
                            arg: Box::new(Expression::new(
                                (2..3).into(),
                                ExpressionKind::Value(Value::new(
                                    (2..3).into(),
                                    ValueKind::Identifier(Identifier::new(
                                        (2..3).into(),
                                        "g".to_string()
                                    )),
                                )),
                            )),
                        }
                    )),
                    arg: Box::new(Expression::new(
                        (4..5).into(),
                        ExpressionKind::Value(Value::new(
                            (4..5).into(),
                            ValueKind::Identifier(Identifier::new((4..5).into(), "h".to_string())),
                        )),
                    )),
                }
            )
        )
    }
}
