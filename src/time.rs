#![allow(unused, clippy::missing_errors_doc)]

use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

pub const CUSTOM_FORMAT: &str = "%Y/%m/%d %H:%M";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FakeUtcTime {
    timestamp: i64,
}

pub type FakeUtcFields = DateTime<Utc>;

impl FakeUtcTime {
    #[must_use]
    pub fn now() -> Self {
        Local::now().naive_local().and_utc().timestamp().into()
    }

    #[must_use]
    pub fn from_timestamp(timestamp: i64) -> Self {
        timestamp.into()
    }

    #[must_use]
    pub fn from_fields(fields: FakeUtcFields) -> Self {
        fields.into()
    }

    pub fn parse_str(time_str: &str, format: &str) -> Result<Self, String> {
        NaiveDateTime::parse_from_str(time_str, format)
            .map(|res| res.and_utc().timestamp().into())
            .map_err(|err| err.to_string())
    }

    pub fn get_fields(&self) -> Result<FakeUtcFields, String> {
        DateTime::from_timestamp(self.timestamp, 0)
            .ok_or(format!("unable to parse timestamp: {}", self.timestamp))
    }

    pub fn format_str(&self, format: &str) -> Result<String, String> {
        self.get_fields()
            .map(|date| date.format(format).to_string())
    }
}

impl From<i64> for FakeUtcTime {
    fn from(value: i64) -> Self {
        Self { timestamp: value }
    }
}

impl From<FakeUtcFields> for FakeUtcTime {
    fn from(value: FakeUtcFields) -> Self {
        Self {
            timestamp: value.timestamp(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use chrono::{Datelike, Days, TimeDelta, Timelike};

    use super::{CUSTOM_FORMAT, FakeUtcTime};

    #[test]
    pub fn time_fields() {
        let fake_utc_time = FakeUtcTime::from_timestamp(1_743_044_280);
        let fields = fake_utc_time.get_fields().unwrap();
        assert_eq!(fields.year(), 2025);
        assert_eq!(fields.month(), 3);
        assert_eq!(fields.day(), 27);
        assert_eq!(fields.hour(), 2);
        assert_eq!(fields.minute(), 58);
    }

    #[test]
    pub fn parse_format_time() {
        let fake_utc_time = FakeUtcTime::from_timestamp(1_743_044_280);
        let date_str = fake_utc_time.format_str(CUSTOM_FORMAT).unwrap();
        assert_eq!(date_str, "2025/03/27 02:58");
        let fake_utc_time2 = FakeUtcTime::parse_str(&date_str, CUSTOM_FORMAT).unwrap();
        assert_eq!(fake_utc_time, fake_utc_time2);
    }
}
