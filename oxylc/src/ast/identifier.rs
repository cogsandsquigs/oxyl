use errgonomic::parser::input::Span;

use super::AstNode;

/// An identifier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identifier {
    /// The name
    name: String,

    /// The location of the identifier.
    location: Span,
}

impl Identifier {
    /// Creates a new `Identifier` object.
    pub fn new(name: String, location: Span) -> Self {
        Self { name, location }
    }

    /// Gets the name of the identifier.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl AstNode for Identifier {
    fn location(&self) -> &Span {
        &self.location
    }
}
