#![allow(unused)]

use chrono::{DateTime, Local};

pub const CUSTOM_FORMAT: &str = "%Y/%m/%d %H:%M";

pub fn now_fake_utc() -> i64 {
    Local::now().naive_local().and_utc().timestamp()
}

pub fn format_str(timestamp: i64, format: &str) -> Result<String, String> {
    DateTime::from_timestamp(timestamp, 0)
        .map(|date| date.format(format).to_string())
        .ok_or(String::from("todo"))
}
