use crate::fst::comment::Comment;

use super::{errors::ParserError, utils::line_ending};
use errgonomic::{
    combinators::{any, is, take_until},
    parser::{errors::Result, state::State, Parser},
};

/// A comment.
pub fn comment(state: State<&str, ParserError>) -> Result<&str, Comment, ParserError> {
    any((single_line_comment, multi_line_comment)).process(state)
}

fn single_line_comment(state: State<&str, ParserError>) -> Result<&str, Comment, ParserError> {
    is("//")
        .then(take_until(line_ending))
        .map(|(x, (y, z))| {
            let comment = x.join(&y).join(&z);
            Comment::new(comment.span(), comment.as_inner().to_string())
        })
        .process(state)
}

fn multi_line_comment(state: State<&str, ParserError>) -> Result<&str, Comment, ParserError> {
    is("/*")
        .then(take_until(is("*/")))
        .map(|(x, (y, z))| {
            let comment = x.join(&y).join(&z);
            Comment::new(comment.span(), comment.as_inner().to_string())
        })
        .process(state)
}
