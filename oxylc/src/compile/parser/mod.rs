mod errors;

use errors::Error;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "compile/parser/oxyl.pest"]
struct OxylParser;

pub fn parse(input: &str) -> Result<(), Error> {
    let result = OxylParser::parse(Rule::FILE, input);
    todo!()
}
