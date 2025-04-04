mod span;

pub mod file;
pub mod identifier;
pub mod value;

use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

pub use span::Span;

/// A type representing a node in the AST. Allows us to transparently reference it's location
/// without including it in the AST structure itself, at least explicity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<N: NodeType> {
    /// The inner node type.
    inner: N,

    /// A span representing the location of the node in the source code.
    span: Span,
}

pub trait NodeType: Debug + Clone + PartialEq + Eq {}

impl<N: NodeType> Node<N> {
    /// Creates a new node with the given inner value and span.
    pub fn new<S: Into<Span>>(inner: N, span: S) -> Self {
        Node {
            inner,
            span: span.into(),
        }
    }

    /// Returns the span (what is taken up by the node) of the node.
    pub fn span(&self) -> Span {
        self.span
    }
}

impl<N: NodeType> Deref for Node<N> {
    type Target = N;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<N: NodeType> DerefMut for Node<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
