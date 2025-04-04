use super::NodeType;

/// A raw value in the Oxyl language
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    /// An integer value.
    Integer(i64),
}

impl NodeType for Value {}
