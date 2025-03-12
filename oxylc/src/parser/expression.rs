use super::{errors::ParserError, utils::parenthesized, value::value};
use crate::ast::{
    expression::{Expression, ExpressionKind},
    AstNode,
};
use errgonomic::{
    combinators::{any, whitespace_wrapped as ww},
    parser::{errors::Result, state::State, Parser},
};

/// Parses an `Expression` object.
/// ```bnf
/// <expression> ::= ( "(" <expression> ")" ) | <value>
/// ```
pub fn expression(state: State<&str, ParserError>) -> Result<&str, Expression, ParserError> {
    any((
        value.map(|value| {
            let location = *value.location();
            Expression::new(ExpressionKind::Value(value), location)
        }),
        parenthesized(expression), // Paren expression
        // NOTE: This must come last, because otherwise we recurse forever
        ww(expression), // Expression wrapped in whitespace
    ))
    .process(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::value::{Value, ValueKind};

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
        assert_eq!(state.as_input().as_inner(), "");
    }
}
