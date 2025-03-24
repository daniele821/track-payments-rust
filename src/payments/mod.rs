use std::collections::{BTreeMap, HashSet};

#[derive(Debug)]
struct Order {
    unit_price: u32,
    quantity: u32,
    item: String,
}

#[derive(Debug)]
struct Payment {
    city: String,
    method: String,
    shop: String,
    date: i64,
    orders: BTreeMap<String, Order>,
}

#[derive(Debug)]
struct AllPayments {
    cities: HashSet<String>,
    shops: HashSet<String>,
    methods: HashSet<String>,
    items: HashSet<String>,
    payments: BTreeMap<i64, Payment>,
}
