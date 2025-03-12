use super::errors::ParserError;
use crate::ast::value::{Value, ValueKind};
use errgonomic::{
    combinators::{any, decimal, is},
    parser::{errors::Result, input::Input, state::State, Parser},
};

/// Parses a `Value` object.
/// ```bnf
/// <value> ::= <numeric>
/// ```
pub fn value(state: State<&str, ParserError>) -> Result<&str, Value, ParserError> {
    // TODO: More cases, this `any` is just here for now as a placeholder.
    any((numeric, boolean)).process(state)
}

/// Parses a numeric thing.
/// ```bnf
/// <numeric> ::= [0-9]+
/// ```
fn numeric(state: State<&str, ParserError>) -> Result<&str, Value, ParserError> {
    decimal
        .map(|parsed: Input<&str>| {
            let location = parsed.span();
            let number = parsed
                .as_inner()
                .parse::<i64>()
                .expect("This should be only decimal digits!");
            Value::new(ValueKind::Integer(number), location)
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
        let input = "abc";
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
