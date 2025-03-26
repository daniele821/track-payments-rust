#![allow(unused)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueSet {
    cities: BTreeSet<String>,
    shops: BTreeSet<String>,
    methods: BTreeSet<String>,
    items: BTreeSet<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    item: String,
    unit_price: u32,
    quantity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    date: i64,
    city: String,
    method: String,
    shop: String,
    orders: Vec<Order>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllPayments {
    value_set: ValueSet,
    payments: Vec<Payment>,
}

impl AllPayments {
    pub fn from_json(json_str: &str) -> Result<Self, String> {
        serde_json::from_str(json_str).map_err(|err| err.to_string())
    }

    pub fn dump_json(&self, fmt: bool) -> Result<String, String> {
        if fmt {
            serde_json::to_string_pretty(self).map_err(|err| err.to_string())
        } else {
            serde_json::to_string(self).map_err(|err| err.to_string())
        }
    }
}
