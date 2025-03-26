#![allow(unused)]

use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

mod json_default;
mod json_legacy;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Getters)]
pub struct ValueSet {
    cities: BTreeSet<String>,
    shops: BTreeSet<String>,
    methods: BTreeSet<String>,
    items: BTreeSet<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Getters)]
pub struct OrderId {
    item: String,
    unit_price: u32,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Getters)]
pub struct OrderDetail {
    quantity: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Getters)]
pub struct PaymentId {
    date: i64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Getters)]
pub struct PaymentDetail {
    city: String,
    shop: String,
    method: String,
    orders: BTreeMap<OrderId, OrderDetail>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Getters)]
pub struct AllPayments {
    value_set: ValueSet,
    payments: BTreeMap<PaymentId, PaymentDetail>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AllPaymentsError {
    OrderDuplicated,
    OrderNotFound,
    PaymentDuplicated,
    PaymentNotFound,
}

impl ValueSet {
    pub fn new() -> Self {
        Self {
            cities: BTreeSet::new(),
            shops: BTreeSet::new(),
            methods: BTreeSet::new(),
            items: BTreeSet::new(),
        }
    }
}

impl OrderId {
    pub fn new(item: String, unit_price: u32) -> Self {
        Self { item, unit_price }
    }
}

impl OrderDetail {
    pub fn new(quantity: u32) -> Self {
        Self { quantity }
    }
}

impl PaymentId {
    pub fn new(date: i64) -> Self {
        Self { date }
    }
}

impl PaymentDetail {
    pub fn new(city: String, shop: String, method: String) -> Self {
        Self {
            city,
            shop,
            method,
            orders: BTreeMap::new(),
        }
    }

    pub fn total_price(&self) -> u32 {
        let mut acc = 0;
        for order in &self.orders {
            acc += order.0.unit_price * order.1.quantity;
        }
        acc
    }
}

impl AllPayments {
    pub fn new() -> Self {
        Self {
            value_set: ValueSet::new(),
            payments: BTreeMap::new(),
        }
    }
}
