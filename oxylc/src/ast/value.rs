use super::{block::Block, function::Function, identifier::Identifier, Node, NodeType};

/// A raw value in the Oxyl language
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    /// An integer value.
    Integer(i64),

    /// An identifier, a reference to something.
    Identifier(Node<Identifier>),

    /// A block.
    Block(Node<Block>),

    /// A function.
    Function(Node<Function>),
}

impl NodeType for Value {}
