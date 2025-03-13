use super::FstNode;
use errgonomic::parser::input::Span;

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
    pub fn new(location: Span, name: String) -> Self {
        Self { name, location }
    }

    /// Gets the name of the identifier.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl FstNode for Identifier {
    fn location(&self) -> &Span {
        &self.location
    }
}
