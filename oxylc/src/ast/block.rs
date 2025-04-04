use super::{expression::Expression, statement::Statement, Node, NodeType};

/// A block in the Oxyl language
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    /// The statements that make up the block
    stmts: Vec<Node<Statement>>,

    /// The expression the block "returns"
    expr: Box<Node<Expression>>,
}

impl Block {
    /// Creates a new block with the given statements and expression
    pub fn new(stmts: Vec<Node<Statement>>, expr: Node<Expression>) -> Self {
        Block {
            stmts,
            expr: Box::new(expr),
        }
    }

    /// Returns the statements of the block
    pub fn stmts(&self) -> &[Node<Statement>] {
        &self.stmts
    }

    /// Returns the expression of the block
    pub fn expr(&self) -> &Node<Expression> {
        &self.expr
    }
}

impl NodeType for Block {}
