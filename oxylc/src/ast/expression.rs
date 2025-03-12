use super::{value::Value, AstNode};
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
        Self { kind, location }
    }

    /// Gets the kind of value it is.
    pub fn kind(&self) -> &ExpressionKind {
        &self.kind
    }
}

impl AstNode for Expression {
    fn location(&self) -> &Span {
        &self.location
    }
}

/// The kinds of expressions we can have.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpressionKind {
    Value(Value),
}
