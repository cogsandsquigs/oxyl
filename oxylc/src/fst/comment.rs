use super::FstNode;
use errgonomic::parser::input::Span;
use std::array;

pub struct Comment {
    /// The comment contents
    comment: String,

    /// The location of the comment
    location: Span,
}

impl Comment {
    pub fn new(comment: String, location: Span) -> Self {
        Self { comment, location }
    }

    pub fn contents(&self) -> &str {
        &self.comment
    }
}

impl FstNode for Comment {
    fn location(&self) -> &Span {
        &self.location
    }

    fn comments(&self) -> &[Comment] {
        array::from_ref(self)
    }
}
