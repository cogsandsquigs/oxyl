use crate::parser::Rule;
use pest::iterators::Pair;

/// Parses a File rule.
/// NOTE: Assuems that `pair.as_rule()` is `Rule::FILE`.
pub fn rule(pair: Pair<'_, Rule>) {
    todo!()
}
