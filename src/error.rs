use chrono::ParseError;
use std::fmt::Display;
use std::result::Result as StdResult;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    TimeParseFailed(#[from] ParseError),
    #[error("{0}")]
    Generic(String),
}

impl Error {
    pub fn from_generic<T: Display>(err: T) -> Self {
        Self::Generic(err.to_string())
    }
}

pub type Result<T> = StdResult<T, Error>;
