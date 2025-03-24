use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::{BTreeSet, HashSet};

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueSet {
    cities: HashSet<String>,
    shops: HashSet<String>,
    methods: HashSet<String>,
    items: HashSet<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    unit_price: u32,
    quantity: u32,
    item: String,
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item && self.unit_price == self.unit_price
    }
}
impl Eq for Order {}
impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}
impl Ord for Order {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let item_order = self.item.cmp(&other.item);
        match item_order {
            std::cmp::Ordering::Less | std::cmp::Ordering::Greater => item_order,
            std::cmp::Ordering::Equal => self.unit_price.cmp(&other.unit_price),
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

impl PartialEq for Payment {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date
    }
}
impl Eq for Payment {}
impl PartialOrd for Payment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}
impl Ord for Payment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.date.cmp(&other.date)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllPayments {
    value_set: ValueSet,
    payments: BTreeSet<Payment>,
}

impl AllPayments {
    pub fn to_json(&self) -> Option<String> {
        serde_json::to_string(self).ok()
    }

    pub fn from_json(json: &str) -> Option<Self> {
        serde_json::from_str(json).ok()
    }
}
