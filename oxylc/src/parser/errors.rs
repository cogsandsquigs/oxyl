use super::Rule;

/// The error type for the parser.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Pest(#[from] Box<pest::error::Error<Rule>>),
}

// NOTE: Need to implement `From` for `pest::error::Error<Rule>` to convert it into `Error`, since
// we box the error b/c it's so large.
impl From<pest::error::Error<Rule>> for Error {
    fn from(err: pest::error::Error<Rule>) -> Self {
        Error::Pest(Box::new(err))
    }
}
