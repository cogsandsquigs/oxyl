use super::Rule;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Pest(#[from] pest::error::Error<Rule>),
}
