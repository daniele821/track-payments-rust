#![allow(unused)]

use std::collections::{BTreeMap, BTreeSet};
use serde::{Deserialize, Serialize};

mod json_default;
mod json_legacy;

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueSet {
    cities: BTreeSet<String>,
    shops: BTreeSet<String>,
    methods: BTreeSet<String>,
    items: BTreeSet<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrderId {
    item: String,
    unit_price: u32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderDetail {
    quantity: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct PaymentId {
    date: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDetail {
    city: String,
    method: String,
    shop: String,
    orders: BTreeMap<OrderId, OrderDetail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllPayments {
    value_set: ValueSet,
    payments: BTreeMap<PaymentId, PaymentDetail>,
}
