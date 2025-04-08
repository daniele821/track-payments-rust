use chrono::ParseError;
use std::fmt::Display;
use std::result::Result as StdResult;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("encryption failed")]
    EncryptionFailed,
    #[error("decryption failed")]
    DecryptionFailed,
    #[error(transparent)]
    TimeParseFailed(#[from] ParseError),
    #[error("{0}")]
    Generic(String),
}

pub type Result<T> = StdResult<T, Error>;

impl Error {
    pub fn from_generic<T: Display>(err: T) -> Self {
        Self::Generic(err.to_string())
    }
}
