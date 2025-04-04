use super::{expression::Expression, identifier::Identifier, Node, NodeType};

/// A statement in the Oxyl language
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    /// A "let" statement
    Let {
        name: Node<Identifier>,
        expr: Node<Expression>,
    },
}

impl NodeType for Statement {}
