use crate::{
    ast::{identifier::Identifier, Node},
    parser::Rule,
};
use pest::iterators::Pair;

/// Parses an Identifier rule.
/// NOTE: Assumes that `pair.as_rule()` is `Rule::IDENTIFIER`.
pub fn rule(pair: Pair<'_, Rule>) -> Node<Identifier> {
    Node::new(Identifier::new(pair.as_str()), &pair)
}
