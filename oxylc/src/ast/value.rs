use super::AstNode;
use errgonomic::parser::input::Span;

/// A value in the AST.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Value {
    /// The type of value it is.
    kind: ValueKind,

    /// The location where the value was found.
    location: Span,
}

impl Value {
    /// Creates a new `Value` object.
    pub fn new(kind: ValueKind, location: Span) -> Self {
        Self { kind, location }
    }

    /// Gets the kind of value it is.
    pub fn kind(&self) -> &ValueKind {
        &self.kind
    }
}

impl AstNode for Value {
    fn location(&self) -> &Span {
        &self.location
    }
}

/// The kinds of values that can be in the AST.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValueKind {
    /// An integer value, without any type determination.
    Integer(i64),

    /// A boolean value.
    Boolean(bool),
}
