#![cfg(test)]

use super::*;

#[test]
fn can_parse() {
    let x = parse("let x = 5;");
}

#[test]
fn can_parse_let() {
    let input = r"let x = 5;";
    let result = OxylParser::parse(Rule::LET_STATEMENT, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);

    let input = r"let x = { 1123 };";
    let result = OxylParser::parse(Rule::LET_STATEMENT, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
}

#[test]
fn can_parse_block() {
    let input = r"{let x = 5; let y = 6; asdf}";
    let result = OxylParser::parse(Rule::BLOCK, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
}
