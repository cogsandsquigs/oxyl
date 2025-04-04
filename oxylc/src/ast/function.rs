use super::{expression::Expression, identifier::Identifier, Node, NodeType};

/// A function in the Oxyl language
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    /// The arguments of the function
    args: Vec<Node<Identifier>>,

    /// The expression of the function
    expr: Box<Node<Expression>>,
}

impl Function {
    /// Creates a new function with the given arguments and expression
    pub fn new(args: Vec<Node<Identifier>>, expr: Node<Expression>) -> Self {
        Function {
            args,
            expr: Box::new(expr),
        }
    }

    /// Returns the arguments of the function
    pub fn args(&self) -> &[Node<Identifier>] {
        &self.args
    }

    /// Returns the expression of the function
    pub fn expr(&self) -> &Node<Expression> {
        &self.expr
    }
}

impl NodeType for Function {}
