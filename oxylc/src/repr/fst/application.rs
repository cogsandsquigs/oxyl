use errgonomic::prelude::Span;

use super::{expression::Expression, identifier::Identifier, FstNode};

/// An application of a function to an argument.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Application {
    /// The location
    location: Span,

    /// The function being applied.
    function: Identifier,

    /// The argument being applied to the function. Note that in Oxyl, functions can only take 1
    /// argument. However, since they are left-associative *and* curryable, it is possible to
    /// "simulate" multiple arguments by chaining applications.
    arg: Expression,
}

impl Application {
    /// Creates a new `Application` object.
    pub fn new(location: Span, function: Identifier, arg: Expression) -> Self {
        Self {
            location,
            function,
            arg,
        }
    }

    /// Gets the function being applied.
    pub fn function(&self) -> &Identifier {
        &self.function
    }

    /// Gets the argument being applied to the function.
    pub fn arg(&self) -> &Expression {
        &self.arg
    }
}

impl FstNode for Application {
    fn location(&self) -> &Span {
        &self.location
    }
}
