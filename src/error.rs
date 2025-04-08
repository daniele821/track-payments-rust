use crate::payments::{OrderId, PaymentId, ValueSet};
use chrono::ParseError;
use std::fmt::Display;
use std::result::Result as StdResult;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("payment already present: {0:?}")]
    PaymentDuplicated(PaymentId),
    #[error("payment not found: {0:?}")]
    PaymentNotFound(PaymentId),
    #[error("order already present: {0:?}, {1:?}")]
    OrderDuplicated(PaymentId, OrderId),
    #[error("order not found: {0:?}, {1:?}")]
    OrderNotFound(PaymentId, OrderId),
    #[error("missing values: {0:?}")]
    MissingElements(ValueSet),
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
