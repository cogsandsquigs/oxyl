use super::{/*function::Function*/ comment::Comment, identifier::Identifier, FstNode};
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

impl FstNode for Value {
    fn location(&self) -> &Span {
        &self.location
    }

    fn comments(&self) -> &[Comment] {
        &[]
    }
}

/// The kinds of values that can be in the AST.
#[derive(Clone, Debug, PartialEq)]
pub enum ValueKind {
    /// An integer value, without any type determination.
    Integer(i64),

    /// A floating-point value, without any type determination.
    Floating(f64),

    /// A boolean value.
    Boolean(bool),

    /// An identifier.
    Identifier(Identifier),
    // /// A function.
    // Function(Function),
}

// NOTE: Have to do this b/c of `f64` not implementing `Eq`.
impl Eq for ValueKind {}
