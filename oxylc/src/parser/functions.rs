use super::{errors::ParserError, expression::expression};
use crate::{
    ast::{function::Function, AstNode},
    parser::ident::ident,
};
use errgonomic::{
    combinators::{is, separated, whitespace_wrapped as ww},
    parser::{errors::Result, state::State, Parser},
};

/// Parses a `Function` object.
/// ```bnf
/// <function> ::= "\" <ident> ("," <ident>)* ","? "." <expression>
/// ```
pub fn function(state: State<&str, ParserError>) -> Result<&str, Function, ParserError> {
    is("\\")
        .then(ww(separated(ww(ident), is(","), true)))
        .then(is("."))
        .then(ww(expression))
        .map_with_state(|state, (((start, idents), _), expression)| {
            let location = start.span().union_between(state.as_input().span());
            (state, Function::new(idents, expression, location))
        })
        .process(state)
}

#[cfg(test)]
mod tests {
    use crate::ast::{
        expression::{Expression, ExpressionKind},
        identifier::Identifier,
        value::{Value, ValueKind},
    };

    use super::*;

    #[test]
    fn can_parse_fn_single_arg() {
        let (state, parsed) = function.process("\\  x \n\n. 123 \n".into()).unwrap();
        assert!(state.is_ok());
        assert_eq!(state.as_input().as_inner(), "");
        assert_eq!(
            parsed,
            Function::new(
                vec![Identifier::new("x".to_string(), (3..4).into())],
                Expression::new(
                    ExpressionKind::Value(Value::new(ValueKind::Integer(123), (9..12).into())),
                    (9..12).into()
                ),
                (0..14).into()
            )
        );
    }

    #[test]
    fn can_parse_fn_mutli_arg() {
        let (state, parsed) = function
            .process("\\  x ,\n\t\r\ny_1\n\n. 123 \n".into())
            .unwrap();
        assert!(state.is_ok());
        assert_eq!(state.as_input().as_inner(), "");
        assert_eq!(
            parsed,
            Function::new(
                vec![
                    Identifier::new("x".to_string(), (3..4).into()),
                    Identifier::new("y_1".to_string(), (10..13).into())
                ],
                Expression::new(
                    ExpressionKind::Value(Value::new(ValueKind::Integer(123), (17..20).into())),
                    (17..20).into()
                ),
                (0..22).into()
            )
        );
    }
}
