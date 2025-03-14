use super::{
    block::block,
    errors::ParserError,
    utils::{parenthesized, wnnw},
    value::value,
};
use crate::fst::{
    expression::{Expression, ExpressionKind},
    FstNode,
};
use errgonomic::{
    combinators::any,
    parser::{errors::Result, state::State, Parser},
};

/// Parses an `Expression` object.
/// ```bnf
/// <expression> ::= ( "(" <expression> ")" ) | <value>
/// ```
pub fn expression(state: State<&str, ParserError>) -> Result<&str, Expression, ParserError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fst::{
        block::Block,
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
}
