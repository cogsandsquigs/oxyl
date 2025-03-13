use super::{/*block::Block*/ comment::Comment, value::Value, FstNode};
use errgonomic::parser::input::Span;

/// An expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expression {
    /// The type of expression it is.
    kind: ExpressionKind,

    /// The location where the value was found.
    location: Span,
}

impl Expression {
    /// Creates a new `Expression` object.
    pub fn new(kind: ExpressionKind, location: Span) -> Self {
        todo!("add comments!");
        Self { kind, location }
    }

    /// Gets the kind of value it is.
    pub fn kind(&self) -> &ExpressionKind {
        &self.kind
    }
}

impl FstNode for Expression {
    fn location(&self) -> &Span {
        &self.location
    }

    fn comments(&self) -> &[Comment] {
        todo!()
    }
}

/// The kinds of expressions we can have.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpressionKind {
    Value(Value),
    // Block(Block),
}
