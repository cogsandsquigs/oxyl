pub mod comment;
pub mod expression;
pub mod identifier;
pub mod value;

/*
pub mod block;
pub mod function;
pub mod statement;
*/

use comment::Comment;
use errgonomic::parser::input::Span;

pub trait FstNode {
    /// Gets the location of the `FstNode`, as a `Span`.
    fn location(&self) -> &Span;

    /// Gets the comments in the FST
    fn comments(&self) -> &[Comment];
}
