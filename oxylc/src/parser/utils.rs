use super::{comments::comment, errors::ParserError};
use errgonomic::{
    combinators::{
        any, between, commit, consumed, eoi, is, many, newlines, whitespace, whitespace_not_newline,
    },
    parser::{errors::Result, input::Input, state::State, Parser},
};

/// A line ending
/// ```bnf
/// <line_ending> ::= ( NEWLINE | EOI )
/// ```
pub fn line_ending(state: State<&str, ParserError>) -> Result<&str, Input<&str>, ParserError> {
    any((newlines, consumed(eoi))).process(state)
}

/// A parenthesized expression
/// ```bnf
/// <parenthesized> ::= "(" your_expression_here ")"
/// ```
/// NOTE: Returns the parens for use in the AST
pub fn parenthesized<'a, O, P: Parser<&'a str, O, ParserError>>(
    p: P,
) -> impl Parser<&'a str, (Input<&'a str>, O, Input<&'a str>), ParserError> {
    is("(")
        .then(commit(p.then(is(")"))))
        .map(|(p1, (o, p2))| (p1, o, p2))
}

/// Shorthand for our modified `whitespace_wrapped`, but includes comments
pub fn ww<'a, O, P: Parser<&'a str, O, ParserError>>(p: P) -> impl Parser<&'a str, O, ParserError> {
    between(
        many(any((comment, whitespace))),
        p,
        many(any((comment, whitespace))),
    )
}

/// Shorthand for our modified `whitespace_not_newline_wrapped`, but includes comments
pub fn wnnw<'a, O, P: Parser<&'a str, O, ParserError>>(
    p: P,
) -> impl Parser<&'a str, O, ParserError> {
    between(
        many(any((comment, whitespace_not_newline))),
        p,
        many(any((comment, whitespace_not_newline))),
    )
}

#[cfg(test)]
mod tests {
    use errgonomic::parser::errors::{Error, ErrorKind, ExpectedError};

    use super::*;

    #[test]
    fn can_parse_line_end() {
        let (state, parsed) = line_ending.process("\n".into()).unwrap();
        assert_eq!(parsed.as_inner(), "\n");
        assert!(state.is_ok());
        assert_eq!(state.as_input().as_inner(), "");

        let (state, parsed) = line_ending.process("".into()).unwrap();
        assert_eq!(parsed.as_inner(), "");
        assert!(state.is_ok());
        assert_eq!(state.as_input().as_inner(), "");

        let (state, parsed) = line_ending.process("\r\n".into()).unwrap();
        assert_eq!(parsed.as_inner(), "\r\n");
        assert!(state.is_ok());
        assert_eq!(state.as_input().as_inner(), "");

        let state = line_ending.process("test".into()).unwrap_err();
        assert_eq!(state.as_input().as_inner(), "test");
        assert_eq!(
            state.errors(),
            &Error::new(
                ErrorKind::all(vec![
                    Error::new(
                        ErrorKind::expected(ExpectedError::Newlines),
                        Input::new_with_span("test", 0..1)
                    ),
                    Error::new(
                        ErrorKind::expected(ExpectedError::Nothing),
                        Input::new_with_span("test", 0..4)
                    )
                ]),
                Input::new_with_span("test", 0..4)
            )
        );
    }
}
