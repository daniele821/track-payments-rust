use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::{BTreeMap, HashSet};

#[derive(Debug, Serialize, Deserialize)]
struct Order {
    unit_price: u32,
    quantity: u32,
    item: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Payment {
    city: String,
    method: String,
    shop: String,
    date: i64,
    orders: BTreeMap<String, Order>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AllPayments {
    cities: HashSet<String>,
    shops: HashSet<String>,
    methods: HashSet<String>,
    items: HashSet<String>,
    payments: BTreeMap<i64, Payment>,
}

pub fn testing() {
    let order = Order {
        unit_price: 34,
        quantity: 12,
        item: String::from("12"),
    };
    let mut payment = Payment {
        city: String::from("12"),
        method: String::from("12"),
        shop: String::from("12"),
        date: 0,
        orders: BTreeMap::new(),
    };
    payment.orders.insert("12".to_string(), order);
    println!("{payment:?}");
    let payment_json = serde_json::to_string_pretty(&payment).unwrap();
    println!("{payment_json}");
}
