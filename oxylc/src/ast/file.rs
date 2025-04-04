use super::{statement::Statement, Node, NodeType};

/// A file in the Oxyl language
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    /// The statements in the file
    stmts: Vec<Node<Statement>>,
}

impl File {
    /// Creates a new file with the given statements
    pub fn new(stmts: Vec<Node<Statement>>) -> Self {
        File { stmts }
    }

    /// Returns the statements in the file
    pub fn stmts(&self) -> &[Node<Statement>] {
        &self.stmts
    }
}

impl NodeType for File {}
