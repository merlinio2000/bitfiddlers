use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("couldn't parse <{what}> in base {base}: {cause:?}")]
    Parsing {
        what: String,
        base: usize,
        cause: ParseIntError,
    },
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
