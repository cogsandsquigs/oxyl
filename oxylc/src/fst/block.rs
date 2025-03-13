use super::{expression::Expression, statement::Statement, FstNode};
use errgonomic::parser::input::Span;

/// An expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    location: Span,
    statements: Vec<Statement>,
    expression: Box<Expression>,
}

impl Block {
    /// Creates a new `Block` object.
    pub fn new(location: Span, statements: Vec<Statement>, expression: Box<Expression>) -> Self {
        Self {
            location,
            statements,
            expression,
        }
    }

    /// Gets the statements in the block.
    pub fn statements(&self) -> &[Statement] {
        &self.statements
    }

    /// Gets the expression in the block.
    pub fn expression(&self) -> &Expression {
        &self.expression
    }
}

impl FstNode for Block {
    fn location(&self) -> &Span {
        &self.location
    }
}
