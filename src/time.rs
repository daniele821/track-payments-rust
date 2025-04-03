#![allow(unused, clippy::missing_errors_doc)]

use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};

pub const CUSTOM_FORMAT: &str = "%Y/%m/%d %H:%M";

#[must_use]
pub fn now_fake_utc() -> i64 {
    Local::now().naive_local().and_utc().timestamp()
}

pub fn get_fields(timestamp: i64) -> Result<DateTime<Utc>, String> {
    DateTime::from_timestamp(timestamp, 0).ok_or(format!("unable to parse timestamp: {timestamp}"))
}

pub fn format_str(timestamp: i64, format: &str) -> Result<String, String> {
    get_fields(timestamp).map(|date| date.format(format).to_string())
}

pub fn parse_str(time_str: &str, format: &str) -> Result<i64, String> {
    NaiveDateTime::parse_from_str(time_str, format)
        .map(|res| res.and_utc().timestamp())
        .map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Timelike};

    use super::{CUSTOM_FORMAT, format_str, get_fields, parse_str};

    #[test]
    pub fn time_fields() {
        let timestamp = 1_743_044_280;
        let fields = get_fields(timestamp).unwrap();
        assert_eq!(fields.year(), 2025);
        assert_eq!(fields.month(), 3);
        assert_eq!(fields.day(), 27);
        assert_eq!(fields.hour(), 2);
        assert_eq!(fields.minute(), 58);
    }

    #[test]
    pub fn parse_format_time() {
        let timestamp = 1_743_044_280;
        let date_str = format_str(timestamp, CUSTOM_FORMAT).unwrap();
        assert_eq!(date_str, "2025/03/27 02:58");
        let timestamp2 = parse_str(&date_str, CUSTOM_FORMAT).unwrap();
        assert_eq!(timestamp2, timestamp);
    }
}
