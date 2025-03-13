use super::{expression::Expression, identifier::Identifier, FstNode};
use errgonomic::parser::input::Span;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function {
    location: Span,
    args: Vec<Identifier>,
    expression: Box<Expression>,
}

impl Function {
    pub fn new(location: Span, args: Vec<Identifier>, expression: Expression) -> Self {
        Self {
            location,
            args,
            expression: Box::new(expression),
        }
    }

    pub fn args(&self) -> &[Identifier] {
        &self.args
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }
}

impl FstNode for Function {
    fn location(&self) -> &Span {
        &self.location
    }
}
