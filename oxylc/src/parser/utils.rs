use super::errors::ParserError;
use errgonomic::{
    combinators::{any, eoi, newlines},
    parser::{errors::Result, state::State, Parser},
};

pub fn line_ending(state: State<&str, ParserError>) -> Result<&str, (), ParserError> {
    any((newlines.map(|_| ()), eoi)).process(state)
}
