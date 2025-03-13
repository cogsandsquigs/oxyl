use super::{expression::Expression, identifier::Identifier, AstNode};
use errgonomic::parser::input::Span;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function {
    args: Vec<Identifier>,
    expression: Box<Expression>,

    location: Span,
}

impl Function {
    pub fn new(args: Vec<Identifier>, expression: Expression, location: Span) -> Self {
        Self {
            args,
            expression: Box::new(expression),
            location,
        }
    }

    pub fn args(&self) -> &[Identifier] {
        &self.args
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }
}

impl AstNode for Function {
    fn location(&self) -> &Span {
        &self.location
    }
}
