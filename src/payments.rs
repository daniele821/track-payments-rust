#![allow(unused)]

use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};

#[derive(Debug, Serialize, Deserialize)]
struct ValueSet {
    cities: BTreeSet<String>,
    shops: BTreeSet<String>,
    methods: BTreeSet<String>,
    items: BTreeSet<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    unit_price: u32,
    quantity: u32,
    item: String,
}

impl Order {
    pub fn new(unit_price: u32, quantity: u32, item: &str) -> Self {
        Self {
            unit_price,
            quantity,
            item: String::from(item),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    city: String,
    method: String,
    shop: String,
    date: i64,
    orders: BTreeSet<Order>,
}

impl Payment {
    pub fn new(city: &str, shop: &str, method: &str, date: i64) -> Self {
        Self {
            city: String::from(city),
            method: String::from(method),
            shop: String::from(shop),
            date,
            orders: BTreeSet::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllPayments {
    value_set: ValueSet,
    payments: BTreeSet<Payment>,
}

impl AllPayments {
    pub fn to_json(&self, pretty: bool) -> Result<String, String> {
        if pretty {
            serde_json::to_string_pretty(self).map_err(|err| err.to_string())
        } else {
            serde_json::to_string(self).map_err(|err| err.to_string())
        }
    }

    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|err| err.to_string())
    }

    pub fn get_payments(&self) -> &BTreeSet<Payment> {
        &self.payments
    }
}
