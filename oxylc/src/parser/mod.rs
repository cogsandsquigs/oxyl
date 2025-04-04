mod errors;
mod rules;
mod tests;

use errors::Error;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/oxyl.pest"]
struct OxylParser;

pub fn parse(input: &str) -> Result<(), Error> {
    let pairs = OxylParser::parse(Rule::FILE, input)?;
    todo!()
}
