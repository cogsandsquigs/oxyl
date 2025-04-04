mod errors;
mod rules;
mod tests;

use errors::Error;
use pest::Parser;
use pest_derive::Parser;

use crate::ast::{file::File, Node};

#[derive(Parser)]
#[grammar = "parser/oxyl.pest"]
struct OxylParser;

pub fn parse(input: &str) -> Result<Node<File>, Error> {
    let mut pairs = OxylParser::parse(Rule::FILE, input)?;
    Ok(rules::file(pairs.next().expect("A file is required")))
}
