use super::{block::Block, value::Value, FstNode};
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
    pub fn new(location: Span, kind: ExpressionKind) -> Self {
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
}

/// The kinds of expressions we can have.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpressionKind {
    /// A parenthesized expression
    Parenthesized {
        lparen_location: Span,
        rparen_location: Span,
        inner: Box<Expression>,
    },

    /// A value
    Value(Value),

    /// A block
    Block(Block),
}
