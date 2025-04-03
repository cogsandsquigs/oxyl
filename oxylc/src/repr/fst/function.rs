use super::{expression::Expression, identifier::Identifier, FstNode};
use errgonomic::parser::input::Span;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function {
    /// The location where the value was found.
    pub location: Span,

    /// The "arguments" to the function (even though they're not really arguments, and every
    /// function actually takes 1 argument).
    pub args: Vec<Identifier>,

    /// The expression to evaluate within the function.
    pub expression: Box<Expression>,
}

impl Function {
    pub fn new(location: Span, args: Vec<Identifier>, expression: Expression) -> Self {
        Self {
            location,
            args,
            expression: Box::new(expression),
        }
    }
}

impl FstNode for Function {
    fn location(&self) -> &Span {
        &self.location
    }
}
