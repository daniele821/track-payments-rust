use std::error::Error as StdError;
use std::result::Result as StdResult;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Generic(Box<dyn StdError>),
}

impl Error {
    pub fn from_generic<T: StdError + 'static>(err: T) -> Self {
        Self::Generic(Box::new(err))
    }
}

pub type Result<T> = StdResult<T, Error>;
