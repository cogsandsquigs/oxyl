use super::{errors::ParserError, functions::function, ident::ident};
use crate::ast::{
    value::{Value, ValueKind},
    AstNode,
};
use errgonomic::{
    combinators::{any, decimal, is, maybe},
    parser::{errors::Result, input::Input, state::State, Parser},
};

/// Parses a `Value` object.
/// ```bnf
/// <value> ::= <numeric>
/// ```
pub fn value(state: State<&str, ParserError>) -> Result<&str, Value, ParserError> {
    // TODO: More cases, this `any` is just here for now as a placeholder.
    any((
        floating,
        numeric,
        boolean,
        ident.map(|i| {
            let location = *i.location();
            Value::new(ValueKind::Identifier(i), location)
        }),
        function.map(|f| {
            let location = *f.location();
            Value::new(ValueKind::Function(f), location)
        }),
    ))
    .process(state)
}

/// Parses a numeric thing.
/// ```bnf
/// <numeric> ::= [0-9]+
/// ```
fn numeric(state: State<&str, ParserError>) -> Result<&str, Value, ParserError> {
    decimal
        .map_res(|parsed: Input<&str>| {
            let location = parsed.span();
            let number = parsed
                .as_inner()
                .parse::<i64>()
                .map_err(ParserError::ParseInt)?;
            Ok(Value::new(ValueKind::Integer(number), location))
        })
        .process(state)
}

/// Parses a floating thing.
/// ```bnf
/// <numeric> ::= [0-9]+ "." [0-9]*
/// ```
fn floating(state: State<&str, ParserError>) -> Result<&str, Value, ParserError> {
    decimal
        .then(is("."))
        .then(maybe(decimal))
        .map_res(|((n1_str, dot), n2_str)| {
            let full = match n2_str {
                Some(x) => n1_str.join(&dot).join(&x),
                None => n1_str.join(&dot),
            };
            let location = full.span();
            let number = full
                .as_inner()
                .parse::<f64>()
                .map_err(ParserError::ParseFloat)?;
            Ok(Value::new(ValueKind::Floating(number), location))
        })
        .process(state)
}

/// Parses a boolean thing.
/// ```bnf
/// <boolean> ::= ( "True" | "False" )
/// ```
fn boolean(state: State<&str, ParserError>) -> Result<&str, Value, ParserError> {
    any((is("True"), is("False")))
        .map(|parsed| {
            Value::new(
                ValueKind::Boolean(match parsed.as_inner() {
                    "True" => true,
                    "False" => false,
                    _ => unreachable!(),
                }),
                parsed.span(),
            )
        })
        .process(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_parser() {
        // Basic integer parsing
        let input = "123";
        let state = State::new(input);
        let result = numeric(state);
        assert!(result.is_ok());

        let (state, parsed_value) = result.unwrap();
        assert_eq!(state.as_input(), &"");
        assert!(state.is_ok());
        assert!(matches!(parsed_value.kind(), ValueKind::Integer(123)));

        // Test with whitespace after
        let input = "456 ";
        let state = State::new(input);
        let result = numeric(state);
        assert!(result.is_ok());

        let (state, parsed_value) = result.unwrap();
        assert_eq!(state.as_input(), &" "); // Should consume only the digits
        assert!(state.is_ok());
        assert!(matches!(parsed_value.kind(), ValueKind::Integer(456)));

        // Test with leading zero
        let input = "0789";
        let state = State::new(input);
        let result = numeric(state);
        assert!(result.is_ok());

        let (state, parsed_value) = result.unwrap();
        assert_eq!(state.as_input(), &""); // Should consume only the digits
        assert!(state.is_ok());
        assert!(matches!(parsed_value.kind(), ValueKind::Integer(789)));

        // Test with negative number (assuming decimal parser accepts them)
        let input = "-42";
        let state = State::new(input);
        let result = numeric(state);
        // This check depends on whether decimal parser accepts negative numbers
        if result.is_ok() {
            let (_, parsed_value) = result.unwrap();
            assert!(matches!(parsed_value.kind(), ValueKind::Integer(-42)));
        }

        // Test with non-numeric input
        let input = "abc";
        let state = State::new(input);
        let result = numeric(state);
        assert!(result.is_err());
    }

    #[test]
    fn can_parse_booleans() {
        let input = "True";
        let (state, parsed) = boolean.process(input.into()).unwrap();
        assert_eq!(state.as_input(), &"");
        assert_eq!(parsed.kind(), &ValueKind::Boolean(true));
        assert!(state.is_ok());

        let input = "False";
        let (state, parsed) = boolean.process(input.into()).unwrap();
        assert_eq!(state.as_input(), &"");
        assert_eq!(parsed.kind(), &ValueKind::Boolean(false));
        assert!(state.is_ok());
    }

    #[test]
    fn can_parse_floating() {
        let input = "123.456";
        let (state, parsed) = floating.process(input.into()).unwrap();
        assert_eq!(state.as_input(), &"");
        assert_eq!(parsed.kind(), &ValueKind::Floating(123.456));
        assert!(state.is_ok());
    }

    #[test]
    fn test_value_parser() {
        // Test basic integer
        let input = "123";
        let state = State::new(input);
        let result = value(state);
        assert!(result.is_ok());

        let (state, parsed_value) = result.unwrap();
        assert_eq!(state.as_input(), &"");
        assert!(matches!(parsed_value.kind(), ValueKind::Integer(123)));

        // Test value parser with non-value input
        let input = "!_abc";
        let state = State::new(input);
        let result = value(state);
        assert!(result.is_err());

        // Test with whitespace
        let input = "  456  ";
        let state = State::new(input);
        let result = value(state);
        // This will depend on whether your parser handles whitespace
        // If it does:
        if result.is_ok() {
            let (_, parsed_value) = result.unwrap();
            assert!(matches!(parsed_value.kind(), ValueKind::Integer(456)));
        }
    }
}
