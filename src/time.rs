#![allow(unused)]

use chrono::Local;

pub fn now_fake_utc() -> i64 {
    Local::now().naive_local().and_utc().timestamp()
}
