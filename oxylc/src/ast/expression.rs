use super::{identifier::Identifier, value::Value, Node, NodeType};

/// An expression in the Oxyl language
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    /// A value
    Value(Node<Value>),

    /// A reference to something.
    Reference(Node<Identifier>),
}

impl NodeType for Expression {}
