//! The `fst` module is responsible for representing the FST (Full Syntax Tree) of the Oxyl
//! language. What is a "Full Syntax Tree", you may ask? Well, it is a (syntax) tree that
//! represents the entirety of the parsed file, from code to comments. This way, we can parse the
//! FST once, and then run formatters/LSPs/the actual compiler on it, and therefore save time by
//! only needing 1 parser.

pub mod block;
pub mod expression;
pub mod function;
pub mod identifier;
pub mod statement;
pub mod value;

use errgonomic::parser::input::Span;
use statement::Statement;

pub trait FstNode {
    /// Gets the location of the `FstNode`, as a `Span`.
    fn location(&self) -> &Span;
}

pub struct File {
    statements: Vec<Statement>,
}

impl File {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}
