use super::{expression::Expression, identifier::Identifier, FstNode};
use errgonomic::parser::input::Span;

/// An expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Statement {
    /// The type of statement it is.
    pub kind: StatementKind,

    /// The location where the value was found.
    pub location: Span,
}

impl Statement {
    /// Creates a new `Expression` object.
    pub fn new(location: Span, kind: StatementKind) -> Self {
        Self { kind, location }
    }
}

impl FstNode for Statement {
    fn location(&self) -> &Span {
        &self.location
    }
}

/// The kinds of statements we can have.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StatementKind {
    /// A `let`-statement, representing assignment.
    Let {
        is_mutable: bool,
        ident: Identifier,
        expression: Expression,
    },
}
