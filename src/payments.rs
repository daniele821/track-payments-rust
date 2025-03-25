#![allow(unused)]

use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueSet {
    cities: BTreeSet<String>,
    shops: BTreeSet<String>,
    methods: BTreeSet<String>,
    items: BTreeSet<String>,
}

#[derive(Debug, Serialize, Deserialize, Derivative, PartialEq, Eq, PartialOrd, Ord)]
pub struct Order {
    item: String,
    unit_price: u32,
    quantity: u32,
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

impl ValueSet {
    pub fn new() -> Self {
        Self {
            cities: BTreeSet::new(),
            shops: BTreeSet::new(),
            methods: BTreeSet::new(),
            items: BTreeSet::new(),
        }
    }

    pub fn add_cities(&mut self, cities: &[String]) {
        self.cities.extend(cities.iter().cloned());
    }
    pub fn add_shops(&mut self, shops: &[String]) {
        self.shops.extend(shops.iter().cloned());
    }
    pub fn add_methods(&mut self, methods: &[String]) {
        self.methods.extend(methods.iter().cloned());
    }
    pub fn add_items(&mut self, items: &[String]) {
        self.items.extend(items.iter().cloned());
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

    pub fn extend(&mut self, other: &Self) {
        self.cities.extend(other.cities.iter().cloned());
        self.shops.extend(other.shops.iter().cloned());
        self.methods.extend(other.methods.iter().cloned());
        self.items.extend(other.items.iter().cloned());
    }
}

impl Order {
    pub fn new(item: String, unit_price: u32, quantity: u32) -> Self {
        Self {
            item,
            unit_price,
            quantity,
        }
    }

    pub fn get_item(&self) -> &str {
        &self.item
    }
    pub fn get_unitprice(&self) -> u32 {
        self.unit_price
    }
    pub fn get_quantity(&self) -> u32 {
        self.quantity
    }

    pub fn calculate_price(&self) -> u32 {
        self.unit_price * self.quantity
    }
}

impl Payment {
    pub fn new(date: i64, city: String, shop: String, method: String) -> Self {
        Self {
            date,
            city,
            method,
            shop,
            orders: BTreeSet::new(),
        }
    }

    pub fn get_date(&self) -> i64 {
        self.date
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
    pub fn get_orders(&self) -> &BTreeSet<Order> {
        &self.orders
    }

    pub fn calculate_price(&self) -> u32 {
        let mut acc = 0;
        for order in &self.orders {
            acc += order.calculate_price();
        }
        acc
    }
}

impl AllPayments {
    pub fn new() -> Self {
        Self {
            value_set: ValueSet::new(),
            payments: BTreeSet::new(),
        }
    }

    pub fn get_valueset(&self) -> &ValueSet {
        &self.value_set
    }
    pub fn get_payments(&self) -> &BTreeSet<Payment> {
        &self.payments
    }

    pub fn extend_valueset(&mut self, value_set: &ValueSet) {
        self.value_set.extend(value_set);
    }
}

#[cfg(test)]
mod tests {
    use super::{AllPayments, Order, Payment, ValueSet};

    #[test]
    fn all_payments() {
        let mut value_set = ValueSet::new();
        value_set.add_cities(&[String::from("London")]);
        value_set.add_cities(&[String::from("Paris")]);
        value_set.add_shops(&[String::from("Pub")]);
        value_set.add_methods(&[String::from("Cash")]);
        value_set.add_items(&[String::from("Apples")]);
        value_set.add_items(&[String::from("Tea")]);

        let mut order1 = Order::new(String::from("Apples"), 220, 1);
        let mut order2 = Order::new(String::from("Tea"), 500, 2);
        let mut order3 = Order::new(String::from("Apples"), 200, 3);

        let mut payment1 = Payment::new(
            1,
            String::from("London"),
            String::from("Pub"),
            String::from("Cash"),
        );
        let mut payment2 = Payment::new(
            2,
            String::from("Paris"),
            String::from("Pub"),
            String::from("Cash"),
        );

        let mut all_payments = AllPayments::new();
        all_payments.extend_valueset(&value_set);
    }
}
