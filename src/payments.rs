#![allow(unused)]

use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
struct ValueSet {
    cities: BTreeSet<String>,
    shops: BTreeSet<String>,
    methods: BTreeSet<String>,
    items: BTreeSet<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Order {
    unit_price: u32,
    quantity: u32,
    item: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Payment {
    city: String,
    method: String,
    shop: String,
    date: i64,
    orders: BTreeSet<Order>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct AllPayments {
    value_set: ValueSet,
    payments: BTreeSet<Payment>,
}
