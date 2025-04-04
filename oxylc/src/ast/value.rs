use super::{identifier::Identifier, Node, NodeType};

/// A raw value in the Oxyl language
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    /// An integer value.
    Integer(i64),

    /// A reference to another value/expression/whatever.
    Reference(Node<Identifier>),
}

impl NodeType for Value {}
