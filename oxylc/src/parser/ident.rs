use super::errors::ParserError;
use crate::ast::initial::identifier::Identifier;
use errgonomic::{
    combinators::{alphabetic, alphanumeric, any, is, many},
    parser::{errors::Result, state::State, Parser},
};

/// Parses an ident.
/// ```bnf
/// <ident> ::= ( [a-zA-Z] | "_" ) ( [a-zA-Z0-9] | "_" )*
/// ```
pub fn ident(state: State<&str, ParserError>) -> Result<&str, Identifier, ParserError> {
    any((alphabetic, is("_")))
        .then(many(any((alphanumeric, is("_")))))
        .map(|(part1, part2s)| {
            let part2 = part2s.into_iter().reduce(|acc, x| acc.join(&x));
            let ident = match part2 {
                Some(x) => part1.join(&x),
                None => part1,
            };

            Identifier::new(ident.as_inner().to_string(), ident.span())
        })
        .process(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::initial::AstNode;

    #[test]
    fn can_parse_ident() {
        let input = "abc";
        let state = State::new(input);
        let result = ident(state);
        assert!(result.is_ok());
        let (state, ident) = result.unwrap();
        assert_eq!(ident.name(), "abc");
        assert_eq!(ident.location(), &(0..3).into());
        assert_eq!(state.as_input().as_inner(), "");
    }
}
