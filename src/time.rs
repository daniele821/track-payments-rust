#![allow(unused)]

use chrono::{DateTime, Local};

pub const CUSTOM_FORMAT: &str = "%Y/%m/%d %H:%M";

pub fn now_fake_utc() -> i64 {
    Local::now().naive_local().and_utc().timestamp()
}

pub fn format_str(timestamp: i64, format: &str) -> Result<String, String> {
    DateTime::from_timestamp(timestamp, 0)
        .map(|date| date.format(format).to_string())
        .ok_or(format!("unable to parse timestamp: {timestamp}"))
}

pub fn parse_str(time_str: &str, format: &str) -> Result<i64, String> {
    DateTime::parse_from_str(time_str, format)
        .map(|timestamp| timestamp.timestamp())
        .map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use crate::time::parse_str;

    use super::{CUSTOM_FORMAT, format_str};

    #[test]
    pub fn parse_format_time() {
        let timestamp = 1_743_044_280;
        let date_str = format_str(timestamp, CUSTOM_FORMAT).unwrap();
        assert_eq!(date_str, "2025/03/27 02:58");
        let timestamp2 = parse_str(&date_str, CUSTOM_FORMAT).unwrap();
        assert_eq!(timestamp2, timestamp);
    }
}
