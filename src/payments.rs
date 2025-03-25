#![allow(unused)]

use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};

#[derive(Debug, Serialize, Deserialize)]
struct ValueSet {
    cities: BTreeSet<String>,
    shops: BTreeSet<String>,
    methods: BTreeSet<String>,
    items: BTreeSet<String>,
}

#[derive(Debug, Serialize, Deserialize, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct Order {
    unit_price: u32,
    quantity: u32,
    #[derivative(PartialEq = "ignore", Ord = "ignore", PartialOrd = "ignore")]
    item: String,
}

#[derive(Debug, Serialize, Deserialize, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct Payment {
    date: i64,
    #[derivative(PartialEq = "ignore", Ord = "ignore", PartialOrd = "ignore")]
    city: String,
    #[derivative(PartialEq = "ignore", Ord = "ignore", PartialOrd = "ignore")]
    method: String,
    #[derivative(PartialEq = "ignore", Ord = "ignore", PartialOrd = "ignore")]
    shop: String,
    #[derivative(PartialEq = "ignore", Ord = "ignore", PartialOrd = "ignore")]
    orders: BTreeSet<Order>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllPayments {
    value_set: ValueSet,
    payments: BTreeSet<Payment>,
}
