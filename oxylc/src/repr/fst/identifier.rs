use super::FstNode;
use errgonomic::parser::input::Span;

/// An identifier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identifier {
    /// The name
    pub name: String,

    /// The location of the identifier.
    pub location: Span,
}

impl Identifier {
    /// Creates a new `Identifier` object.
    pub fn new(location: Span, name: String) -> Self {
        Self { name, location }
    }
}

impl FstNode for Identifier {
    fn location(&self) -> &Span {
        &self.location
    }
}
