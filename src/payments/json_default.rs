#![allow(unused)]

use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashSet};

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
