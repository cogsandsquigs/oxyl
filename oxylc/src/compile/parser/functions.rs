use super::{errors::ParserError, expression::expression, ident::ident, utils::ww};
use crate::fst::function::Function;
use errgonomic::{
    combinators::{is, separated},
    parser::{errors::Result, state::State, Parser},
};

/// Parses a `Function` object.
/// ```bnf
/// <function> ::= "\" (<ident> ",")* <ident>? ","? "." <expression>
/// ```
pub fn function(state: State<&str, ParserError>) -> Result<&str, Function, ParserError> {
    is("\\")
        .then(separated(ww(ident), is(","), true))
        .then(ww(is(".")))
        .then(ww(expression))
        .map_with_state(|state, (((start, idents), _), expression)| {
            let location = start.span().union_between(state.as_input().span());
            (state, Function::new(location, idents, expression))
        })
        .process(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fst::{
        expression::{Expression, ExpressionKind},
        identifier::Identifier,
        value::{Value, ValueKind},
    };

    #[test]
    fn can_parse_fn_no_arg() {
        let (state, parsed) = function.process("\\   \n\r\n. 123 \n".into()).unwrap();
        assert!(state.is_ok());
        assert_eq!(state.as_input().as_inner(), "");
        assert_eq!(
            parsed,
            Function::new(
                (0..14).into(),
                vec![],
                Expression::new(
                    (9..12).into(),
                    ExpressionKind::Value(Value::new((9..12).into(), ValueKind::Integer(123),)),
                ),
            )
        );
    }

    #[test]
    fn can_parse_fn_single_arg() {
        let (state, parsed) = function.process("\\  x \n\n. 123 \n".into()).unwrap();
        assert!(state.is_ok());
        assert_eq!(state.as_input().as_inner(), "");
        assert_eq!(
            parsed,
            Function::new(
                (0..14).into(),
                vec![Identifier::new((3..4).into(), "x".to_string(),)],
                Expression::new(
                    (9..12).into(),
                    ExpressionKind::Value(Value::new((9..12).into(), ValueKind::Integer(123),)),
                ),
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
                (0..22).into(),
                vec![
                    Identifier::new((3..4).into(), "x".to_string(),),
                    Identifier::new((10..13).into(), "y_1".to_string(),)
                ],
                Expression::new(
                    (17..20).into(),
                    ExpressionKind::Value(Value::new((17..20).into(), ValueKind::Integer(123),)),
                ),
            )
        );
    }
}
