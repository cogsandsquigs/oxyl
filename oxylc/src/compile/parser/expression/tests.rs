#![cfg(test)]

use super::*;
use crate::repr::fst::{
    block::Block,
    identifier::Identifier,
    value::{Value, ValueKind},
};

#[test]
fn can_parse_value_expression() {
    let (state, expr) = expression.process("123".into()).unwrap();
    assert_eq!(
        expr.kind(),
        &ExpressionKind::Value(Value::new((0..3).into(), ValueKind::Integer(123),))
    );
    assert_eq!(state.as_input().as_inner(), "");
}

#[test]
fn can_parse_parenthesized_expression() {
    let (state, expr) = expression.process("(123)".into()).unwrap();
    assert_eq!(
        expr.kind(),
        &ExpressionKind::Parenthesized {
            lparen_location: (0..1).into(),
            rparen_location: (4..5).into(),
            inner: Box::new(Expression::new(
                (1..4).into(),
                ExpressionKind::Value(Value::new((1..4).into(), ValueKind::Integer(123)))
            ))
        }
    );
    assert_eq!(state.as_input().as_inner(), "");
}

#[test]
fn can_parse_whitespace_wrapped_expression() {
    let (state, expr) = expression.process("  ( 123  )   \t\n".into()).unwrap();
    assert_eq!(
        expr.kind(),
        &ExpressionKind::Parenthesized {
            lparen_location: (2..3).into(),
            rparen_location: (9..10).into(),
            inner: Box::new(Expression::new(
                (4..7).into(),
                ExpressionKind::Value(Value::new((4..7).into(), ValueKind::Integer(123)))
            ))
        }
    );
    assert_eq!(state.as_input().as_inner(), "\n");
}

#[test]
fn can_parse_block_expression() {
    let (state, expr) = expression.process("{ 123 }".into()).unwrap();
    assert_eq!(
        expr.kind(),
        &ExpressionKind::Block(Block::new(
            (0..7).into(),
            vec![],
            Box::new(Expression::new(
                (2..5).into(),
                ExpressionKind::Value(Value::new((2..5).into(), ValueKind::Integer(123),)),
            )),
        ))
    );
    assert_eq!(state.as_input().as_inner(), "");
}

#[test]
fn can_parse_triangle_and_dot() {
    let (state, actual) = expression.process("a.b |> c".into()).unwrap();
    let expected = Expression::new(
        (0..8).into(),
        ExpressionKind::Infix {
            operator: Operator::new((4..6).into(), OperatorKind::Triangle),
            lhs: Box::new(Expression::new(
                (0..3).into(),
                ExpressionKind::Infix {
                    operator: Operator::new((1..2).into(), OperatorKind::Dot),
                    lhs: Box::new(Expression::new(
                        (0..1).into(),
                        ExpressionKind::Value(Value::new(
                            (0..1).into(),
                            ValueKind::Identifier(Identifier::new((0..1).into(), "a".into())),
                        )),
                    )),
                    rhs: Box::new(Expression::new(
                        (2..3).into(),
                        ExpressionKind::Value(Value::new(
                            (2..3).into(),
                            ValueKind::Identifier(Identifier::new((2..3).into(), "b".into())),
                        )),
                    )),
                },
            )),
            rhs: Box::new(Expression::new(
                (7..8).into(),
                ExpressionKind::Value(Value::new(
                    (7..8).into(),
                    ValueKind::Identifier(Identifier::new((7..8).into(), "c".into())),
                )),
            )),
        },
    );
    assert_eq!(
        actual, expected,
        "left:\n{:#?}\nright: \n{:#?}",
        actual, expected
    );
    assert_eq!(state.as_input().as_inner(), "");
    assert!(state.is_ok());
}

#[test]
fn can_parse_triangle() {
    let (state, actual) = expression.process("a |> b |> c".into()).unwrap();
    let expected = Expression::new(
        (0..11).into(),
        ExpressionKind::Infix {
            operator: Operator::new((2..4).into(), OperatorKind::Triangle),
            lhs: Box::new(Expression::new(
                (0..1).into(),
                ExpressionKind::Value(Value::new(
                    (0..1).into(),
                    ValueKind::Identifier(Identifier::new((0..1).into(), "a".into())),
                )),
            )),
            rhs: Box::new(Expression::new(
                (5..11).into(),
                ExpressionKind::Infix {
                    operator: Operator::new((7..9).into(), OperatorKind::Triangle),
                    lhs: Box::new(Expression::new(
                        (5..6).into(),
                        ExpressionKind::Value(Value::new(
                            (5..6).into(),
                            ValueKind::Identifier(Identifier::new((5..6).into(), "b".into())),
                        )),
                    )),
                    rhs: Box::new(Expression::new(
                        (10..11).into(),
                        ExpressionKind::Value(Value::new(
                            (10..11).into(),
                            ValueKind::Identifier(Identifier::new((10..11).into(), "c".into())),
                        )),
                    )),
                },
            )),
        },
    );
    assert_eq!(
        actual, expected,
        "left:\n{:#?}\nright: \n{:#?}",
        actual, expected
    );
    assert_eq!(state.as_input().as_inner(), "");
    assert!(state.is_ok());
}
