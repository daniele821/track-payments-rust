use std::collections::BTreeMap;

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
    payments: BTreeMap<i64, Payment>,
}
