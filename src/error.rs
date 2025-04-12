use crate::{
    payments::{OrderId, PaymentId, ValueSet},
    time::FakeUtcTime,
};
use chrono::ParseError;
use serde_json::Error as JsonError;
use std::fmt::Display;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
    PaymentDuplicated(PaymentId),
    PaymentNotFound(PaymentId),
    OrderDuplicated(PaymentId, OrderId),
    OrderNotFound(PaymentId, OrderId),
    MissingElements(ValueSet),
    TimeParseFailed(ParseError),
    TimeFormatFailed(FakeUtcTime),
    EncryptionFailed,
    DecryptionFailed,
    JsonParseFailed(JsonError),
    JsonDumpFailed(JsonError),
    FileError(IoError),
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
            Error::PaymentDuplicated(pay) => format!("payment duplicated: {:?}", pay),
            Error::PaymentNotFound(pay) => format!("payment not found: {:?}", pay),
            Error::OrderDuplicated(pay, ord) => format!("order duplicated: {:?}, {ord:?}", pay),
            Error::OrderNotFound(pay, ord) => format!("order not found: {:?}, {ord:?}", pay),
            Error::MissingElements(value_set) => format!("missing values: {value_set:?}"),
            Error::TimeParseFailed(parse_error) => format!("parsing time failed: {parse_error}"),
            Error::TimeFormatFailed(time) => format!("formatting time failed: {:?}", time),
            Error::EncryptionFailed => String::from("encryption failed"),
            Error::DecryptionFailed => String::from("decryption failed"),
            Error::JsonParseFailed(err) => format!("json parsing failed: {err}"),
            Error::JsonDumpFailed(err) => format!("json dumping failed: {err}"),
            Error::FileError(err) => err.to_string(),
            Error::Generic(err) => err.to_string(),
        };
        writeln!(f, "{fmt}")
    }
}

impl std::error::Error for Error {}
