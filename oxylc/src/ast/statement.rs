use super::{expression::Expression, identifier::Identifier, AstNode};
use errgonomic::parser::input::Span;

/// An expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Statement {
    /// The type of statement it is.
    kind: StatementKind,

    /// The location where the value was found.
    location: Span,
}

impl Statement {
    /// Creates a new `Expression` object.
    pub fn new(kind: StatementKind, location: Span) -> Self {
        Self { kind, location }
    }

    /// Gets the kind of value it is.
    pub fn kind(&self) -> &StatementKind {
        &self.kind
    }
}

impl AstNode for Statement {
    fn location(&self) -> &Span {
        &self.location
    }
}

/// The kinds of statements we can have.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StatementKind {
    Let {
        is_mutable: bool,
        ident: Identifier,
        expression: Expression,
    },
}
