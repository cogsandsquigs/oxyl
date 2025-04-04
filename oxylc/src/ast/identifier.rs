use super::NodeType;

/// An identifier in the Oxyl language.
#[derive(Debug, Clone, PartialEq, Eq, NodeType)]
pub struct Identifier {
    /// The name of the identifier.
    name: String,
}

impl Identifier {
    /// Creates a new identifier from a string.
    pub fn new(name: &str) -> Self {
        Identifier {
            name: name.to_string(),
        }
    }

    /// Returns the name of the identifier.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl NodeType for Identifier {}
