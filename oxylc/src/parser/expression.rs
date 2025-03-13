use super::{block::block, errors::ParserError, utils::parenthesized, utils::wnnw, value::value};
use crate::ast::{
    expression::{Expression, ExpressionKind},
    AstNode,
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
        wnnw(value.map(|value| {
            let location = *value.location();
            Expression::new(ExpressionKind::Value(value), location)
        })),
        wnnw(block.map(|block| {
            let location = *block.location();
            Expression::new(ExpressionKind::Block(block), location)
        })),
        wnnw(parenthesized(expression)), // Paren expression
    ))
    .process(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{
        block::Block,
        value::{Value, ValueKind},
    };

    #[test]
    fn can_parse_value_expression() {
        let (state, expr) = expression.process("123".into()).unwrap();
        assert_eq!(
            expr.kind(),
            &ExpressionKind::Value(Value::new(ValueKind::Integer(123), (0..3).into()))
        );
        assert_eq!(state.as_input().as_inner(), "");
    }

    #[test]
    fn can_parse_parenthesized_expression() {
        let (state, expr) = expression.process("(123)".into()).unwrap();
        assert_eq!(
            expr.kind(),
            &ExpressionKind::Value(Value::new(ValueKind::Integer(123), (1..4).into()))
        );
        assert_eq!(state.as_input().as_inner(), "");
    }

    #[test]
    fn can_parse_whitespace_wrapped_expression() {
        let (state, expr) = expression.process("  ( 123  )   \t\n".into()).unwrap();
        assert_eq!(
            expr.kind(),
            &ExpressionKind::Value(Value::new(ValueKind::Integer(123), (4..7).into()))
        );
        assert_eq!(state.as_input().as_inner(), "\n");
    }

    #[test]
    fn can_parse_block_expression() {
        let (state, expr) = expression.process("{ 123 }".into()).unwrap();
        assert_eq!(
            expr.kind(),
            &ExpressionKind::Block(Block::new(
                vec![],
                Box::new(Expression::new(
                    ExpressionKind::Value(Value::new(ValueKind::Integer(123), (2..5).into())),
                    (2..5).into()
                )),
                (0..7).into()
            ))
        );
        assert_eq!(state.as_input().as_inner(), "");
    }
}
