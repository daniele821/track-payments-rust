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
    TimeParseFailed(ParseError),
    EncryptionFailed,
    DecryptionFailed,
    JsonParseFailed(String),
    JsonDumpFailed(String),
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
        let fmt = |pay: &PaymentId| {
            format!(
                "PaymentId {{ date: {} }}",
                pay.date()
                    .format_str()
                    .unwrap_or(pay.date().timestamp().to_string())
            )
        };
        let fmt = match self {
            Error::PaymentDuplicated(pay) => format!("payment already present: {}", fmt(pay)),
            Error::PaymentNotFound(pay) => format!("payment not found: {}", fmt(pay)),
            Error::OrderDuplicated(pay, ord) => format!("order not found: {}, {ord:?}", fmt(pay)),
            Error::OrderNotFound(pay, ord) => format!("order not found: {}, {ord:?}", fmt(pay)),
            Error::MissingElements(value_set) => format!("missing values: {value_set:?}"),
            Error::TimeParseFailed(parse_error) => format!("parsing time failed: {parse_error}"),
            Error::EncryptionFailed => String::from("encryption failed"),
            Error::DecryptionFailed => String::from("decryption failed"),
            Error::JsonParseFailed(err) => format!("json parsing failed: {err}"),
            Error::JsonDumpFailed(err) => format!("json dumping failed: {err}"),
            Error::Generic(err) => err.to_string(),
        };
        writeln!(f, "{fmt}")
    }
}

impl std::error::Error for Error {}
