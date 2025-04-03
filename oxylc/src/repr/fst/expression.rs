use super::{block::Block, value::Value, FstNode};
use errgonomic::parser::input::Span;

/// An expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expression {
    /// The location where the value was found.
    pub location: Span,

    /// The type of expression it is.
    pub kind: ExpressionKind,
}

impl Expression {
    /// Creates a new `Expression` object.
    pub fn new(location: Span, kind: ExpressionKind) -> Self {
        Self { kind, location }
    }
}

impl FstNode for Expression {
    fn location(&self) -> &Span {
        &self.location
    }
}

/// The kinds of expressions we can have.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpressionKind {
    /// A parenthesized expression
    Parenthesized {
        lparen_location: Span,
        rparen_location: Span,
        inner: Box<Expression>,
    },

    /// A value
    Value(Value),

    /// A block
    Block(Block),

    /// An infix expression.
    Infix {
        operator: Operator,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },

    /// A prefix expression.
    Prefix {
        operator: Operator,
        rhs: Box<Expression>,
    },

    /// A postfix expression.
    Postfix {
        operator: Operator,
        lhs: Box<Expression>,
    },

    /// An application of an expression onto another
    Application {
        /// The function being applied.
        function: Box<Expression>,

        /// The argument being applied to the function. Note that in Oxyl, functions can only take 1
        /// argument. However, since they are left-associative *and* curryable, it is possible to
        /// "simulate" multiple arguments by chaining applications.
        arg: Box<Expression>,
    },
}

/// The operator that we found.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Operator {
    location: Span,
    kind: OperatorKind,
}

impl Operator {
    /// Creates a new operator.
    pub fn new(location: Span, kind: OperatorKind) -> Self {
        Self { location, kind }
    }

    /// Gets the kind of operator it is.
    pub fn kind(&self) -> OperatorKind {
        self.kind
    }
}

impl FstNode for Operator {
    fn location(&self) -> &Span {
        &self.location
    }
}

/// Operators we can have
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OperatorKind {
    /// The `+`
    Plus,

    /// The `-`
    Dash,

    /// The `*`
    Star,

    /// The `/`
    FSlash,

    /// The `|>`
    Triangle,

    /// The `.`
    Dot,

    /// The `::`, A.K.A "Namespaces"
    DoubleColon,

    /// A function application, A.K.A. `f a` (the whitespace)
    Application,
}
