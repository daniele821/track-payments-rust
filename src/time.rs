//! time related utility functions.

#![allow(unused)]

use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};

pub const CUSTOM_FORMAT: &str = "%Y/%m/%d %H:%M";

/// Get current local time as if it was UTC, in unix time.
#[must_use]
pub fn now_fake_utc() -> i64 {
    Local::now().naive_local().and_utc().timestamp()
}

/// Format the timestamp with the format string specified.
///
/// # Errors
///
/// This function will return an error if string formatting fails.
pub fn format_str(timestamp: i64, format: &str) -> Result<String, String> {
    DateTime::from_timestamp(timestamp, 0)
        .map(|date| date.format(format).to_string())
        .ok_or(format!("unable to parse timestamp: {timestamp}"))
}

/// Parse a string with the format string specified, and return a timestamp.
///
/// # Errors
///
/// This function will return an error if string parsing fails.
pub fn parse_str(time_str: &str, format: &str) -> Result<i64, String> {
    NaiveDateTime::parse_from_str(time_str, format)
        .map(|res| res.and_utc().timestamp())
        .map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::{CUSTOM_FORMAT, format_str, parse_str};

    #[test]
    pub fn parse_format_time() {
        let timestamp = 1_743_044_280;
        let date_str = format_str(timestamp, CUSTOM_FORMAT).unwrap();
        assert_eq!(date_str, "2025/03/27 02:58");
        let timestamp2 = parse_str(&date_str, CUSTOM_FORMAT).unwrap();
        assert_eq!(timestamp2, timestamp);
    }
}
