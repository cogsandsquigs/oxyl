#![cfg(test)]

use super::*;
use crate::ast::{
    expression::Expression, identifier::Identifier, statement::Statement, value::Value,
};

#[test]
fn can_parse_empty() {
    let input = r"";
    let result = parse(input).unwrap();
    assert!(result.stmts().is_empty());
}

#[test]
fn can_parse_single_let() {
    let input = r"let x = 5;";
    let result = parse(input).unwrap();
    assert_eq!(result.stmts().len(), 1);
    let Statement::Let { ref name, ref expr } = *result.stmts()[0];
    assert_eq!(name.name(), "x");
    let Expression::Value(ref value) = **expr;
    assert_eq!(**value, Value::Integer(5));
}
