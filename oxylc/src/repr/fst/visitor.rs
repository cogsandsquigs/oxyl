//! The visitor pattern to walk along the FST, and return some "thing" which we want.

use super::{
    block::Block, expression::Expression, function::Function, identifier::Identifier,
    statement::Statement, value::Value, File,
};

/// The `FstVisitor` allows for a program to visit the Fst and do things on it. Here's how it
/// works:
///     1. You implement the `FstVisitor` trait.
///     2. Whenever you traverse a node, you read + do things with it.
///     3. Save the result to the visitor, and allow it to continue visiting.
///     4. This results in the nodes being visited in a depth-first manner, "in-order", in whatever
///        manner you choose!
///
/// NOTE: The `FstVisitor` should call itself to visit to nodes inside whatever we're visiting.
pub trait FstVisitor<T> {
    fn visit_file(&mut self, file: &File) -> T;
    fn visit_statement(&mut self, statement: &Statement) -> T;
    fn visit_expression(&mut self, expression: &Expression) -> T;
    fn visit_value(&mut self, value: &Value) -> T;
    fn visit_ident(&mut self, ident: &Identifier) -> T;
    fn visit_function(&mut self, function: &Function) -> T;
    fn visit_block(&mut self, block: &Block) -> T;
}
