use super::{value::Value, Node, NodeType};

/// An expression in the Oxyl language
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    /// A value
    Value(Node<Value>),
}

impl NodeType for Expression {}
