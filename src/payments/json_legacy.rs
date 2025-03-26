#![allow(unused)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueSet {
    cities: BTreeSet<String>,
    shops: BTreeSet<String>,
    #[serde(rename = "paymentMethods")]
    methods: BTreeSet<String>,
    items: BTreeSet<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    item: String,
    #[serde(rename = "unitPrice")]
    unit_price: u32,
    quantity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    date: String,
    city: String,
    #[serde(rename = "paymentMethod")]
    method: String,
    shop: String,
    orders: Vec<Order>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllPayments {
    #[serde(rename = "valueSet")]
    value_set: ValueSet,
    payments: Vec<Payment>,
}
