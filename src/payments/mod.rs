#![allow(unused)]

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

mod json_default;
mod json_legacy;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrderDetail {
    quantity: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct PaymentId {
    date: i64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PaymentDetail {
    city: String,
    shop: String,
    method: String,
    orders: BTreeMap<OrderId, OrderDetail>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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

    pub fn get_cities(&self) -> &BTreeSet<String> {
        &self.cities
    }
    pub fn get_shops(&self) -> &BTreeSet<String> {
        &self.shops
    }
    pub fn get_methods(&self) -> &BTreeSet<String> {
        &self.methods
    }
    pub fn get_items(&self) -> &BTreeSet<String> {
        &self.items
    }
}

impl OrderId {
    pub fn new(item: String, unit_price: u32) -> Self {
        Self { item, unit_price }
    }

    pub fn get_item(&self) -> &str {
        &self.item
    }
    pub fn get_unitprice(&self) -> u32 {
        self.unit_price
    }
}

impl OrderDetail {
    pub fn new(quantity: u32) -> Self {
        Self { quantity }
    }

    pub fn get_quantity(&self) -> u32 {
        self.quantity
    }
}

impl PaymentId {
    pub fn new(date: i64) -> Self {
        Self { date }
    }

    pub fn get_date(&self) -> i64 {
        self.date
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

    pub fn get_city(&self) -> &str {
        &self.city
    }
    pub fn get_shop(&self) -> &str {
        &self.shop
    }
    pub fn get_method(&self) -> &str {
        &self.method
    }
    pub fn get_orders(&self) -> &BTreeMap<OrderId, OrderDetail> {
        &self.orders
    }
}

impl AllPayments {
    pub fn new() -> Self {
        Self {
            value_set: ValueSet::new(),
            payments: BTreeMap::new(),
        }
    }

    pub fn get_valueset(&self) -> &ValueSet {
        &self.value_set
    }
    pub fn get_payments(&self) -> &BTreeMap<PaymentId, PaymentDetail> {
        &self.payments
    }
}
