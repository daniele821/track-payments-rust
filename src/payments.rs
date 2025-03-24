use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::{BTreeMap, HashSet};

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    unit_price: u32,
    quantity: u32,
    item: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    city: String,
    method: String,
    shop: String,
    date: i64,
    orders: BTreeMap<String, Order>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllPayments {
    cities: HashSet<String>,
    shops: HashSet<String>,
    methods: HashSet<String>,
    items: HashSet<String>,
    payments: BTreeMap<i64, Payment>,
}

impl AllPayments {
    pub fn to_json(&self) -> Option<String> {
        serde_json::to_string(self).ok()
    }

    pub fn from_json(json: &str) -> Option<Self> {
        serde_json::from_str(json).ok()
    }
}
