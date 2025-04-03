use super::{expression::Expression, statement::Statement, FstNode};
use errgonomic::parser::input::Span;

/// An expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    /// The location where the value was found.
    pub location: Span,

    /// The statements in the block to be executed.
    pub statements: Vec<Statement>,

    /// The final expression in the block, that is the "return value".
    pub expression: Box<Expression>,
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
}

impl FstNode for Block {
    fn location(&self) -> &Span {
        &self.location
    }
}
