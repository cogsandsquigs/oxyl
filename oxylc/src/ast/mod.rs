use errgonomic::parser::input::Span;
use statement::Statement;

pub mod block;
pub mod expression;
pub mod identifier;
pub mod statement;
pub mod value;

pub trait AstNode {
    /// Gets the location of the `AstNode`, as a `Span`.
    fn location(&self) -> &Span;
}

pub struct Ast {
    /// The statements in the AST.
    statements: Vec<Statement>,
}

impl Ast {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }

    pub fn statements(&self) -> &[Statement] {
        &self.statements
    }
}
