use super::{errors::ParserError, utils::line_ending};
use errgonomic::{
    combinators::{any, is, take_until},
    parser::{errors::Result, input::Input, state::State, Parser},
};

/// A comment.
pub fn comment(state: State<&str, ParserError>) -> Result<&str, Input<&str>, ParserError> {
    any((single_line_comment, multi_line_comment)).process(state)
}

fn single_line_comment(state: State<&str, ParserError>) -> Result<&str, Input<&str>, ParserError> {
    is("//")
        .then(take_until(line_ending))
        .map(|(x, (y, z))| x.join(&y).join(&z))
        .process(state)
}

fn multi_line_comment(state: State<&str, ParserError>) -> Result<&str, Input<&str>, ParserError> {
    is("/*")
        .then(take_until(is("*/")))
        .map(|(x, (y, z))| x.join(&y).join(&z))
        .process(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_single_line_comment() {
        let (state, parsed) = single_line_comment.process("// hello".into()).unwrap();
        assert_eq!(parsed.as_inner(), "// hello");
        assert!(state.is_ok());
        assert_eq!(state.as_input().as_inner(), "");

        let (state, parsed) = single_line_comment.process("// hello   \n".into()).unwrap();
        assert_eq!(parsed.as_inner(), "// hello   \n");
        assert!(state.is_ok());
        assert_eq!(state.as_input().as_inner(), "");

        let (state, parsed) = single_line_comment
            .process("// hello   \n\n".into())
            .unwrap();
        assert_eq!(parsed.as_inner(), "// hello   \n\n");
        assert!(state.is_ok());
        assert_eq!(state.as_input().as_inner(), "");
    }

    #[test]
    fn can_parse_multi_line_comment() {
        let (state, parsed) = multi_line_comment.process("/* hello */".into()).unwrap();
        assert_eq!(parsed.as_inner(), "/* hello */");
        assert!(state.is_ok());
        assert_eq!(state.as_input().as_inner(), "");
    }
}
