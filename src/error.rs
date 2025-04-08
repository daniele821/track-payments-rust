use crate::payments::{OrderId, PaymentId, ValueSet};
use chrono::ParseError;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    PaymentDuplicated(PaymentId),
    PaymentNotFound(PaymentId),
    OrderDuplicated(PaymentId, OrderId),
    OrderNotFound(PaymentId, OrderId),
    MissingElements(ValueSet),
    EncryptionFailed,
    DecryptionFailed,
    TimeParseFailed(ParseError),
    Generic(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn from_generic<T: Display>(err: T) -> Self {
        Self::Generic(err.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt = match self {
            Error::PaymentDuplicated(pay) => format!("payment already present: {pay:?}"),
            Error::PaymentNotFound(pay) => format!("payment not found: {pay:?}"),
            Error::OrderDuplicated(pay, order) => format!("order not found: {pay:?}, {order:?}"),
            Error::OrderNotFound(pay, order) => format!("order not found: {pay:?}, {order:?}"),
            Error::MissingElements(value_set) => format!("missing values: {value_set:?}"),
            Error::EncryptionFailed => String::from("encryption failed"),
            Error::DecryptionFailed => String::from("decryption failed"),
            Error::TimeParseFailed(parse_error) => format!("parsing time failed: {parse_error}"),
            Error::Generic(err) => err.to_string(),
        };
        writeln!(f, "{fmt}")
    }
}

impl std::error::Error for Error {}
