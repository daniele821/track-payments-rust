#![allow(unused)]

mod trait_impl;

use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum Element {
    City(String),
    Shop(String),
    Method(String),
    Item(String),
}

pub type Elements = BTreeSet<Element>;

#[derive(Debug)]
pub enum PaymentError {
    Generic(String),
    MissingElements(Elements),
    DuplicateElements(Elements),
    DuplicatePayment(Payment),
    DuplicateOrder(Order),
}

#[derive(Debug, Serialize, Deserialize)]
struct ValueSet {
    cities: BTreeSet<String>,
    shops: BTreeSet<String>,
    methods: BTreeSet<String>,
    items: BTreeSet<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    unit_price: u32,
    quantity: u32,
    item: String,
}

impl Order {
    pub fn new(unit_price: u32, quantity: u32, item: String) -> Self {
        Self {
            unit_price,
            quantity,
            item,
        }
    }

    fn validate(&self, value_set: &ValueSet) -> Elements {
        let mut missing_elements = Elements::new();
        let item = &self.item;
        if !value_set.items.contains(item) {
            missing_elements.insert(Element::Item(item.clone()));
        }
        missing_elements
    }

    pub fn get_unitprice(&self) -> u32 {
        self.unit_price
    }
    pub fn get_quantity(&self) -> u32 {
        self.quantity
    }
    pub fn get_item(&self) -> &str {
        &self.item
    }
    pub fn calculate_price(&self) -> u32 {
        self.unit_price * self.quantity
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

impl Payment {
    pub fn new(city: String, method: String, shop: String, date: i64) -> Self {
        Self {
            city,
            method,
            shop,
            date,
            orders: BTreeSet::new(),
        }
    }

    fn validate(&self, value_set: &ValueSet) -> Elements {
        let mut missing_elements = Elements::new();
        let city = &self.city;
        let shop = &self.shop;
        let method = &self.method;
        if !value_set.cities.contains(city) {
            missing_elements.insert(Element::City(city.clone()));
        }
        if !value_set.shops.contains(shop) {
            missing_elements.insert(Element::Shop(shop.clone()));
        }
        if !value_set.methods.contains(method) {
            missing_elements.insert(Element::Method(method.clone()));
        }
        missing_elements
    }

    pub fn get_city(&self) -> &str {
        &self.city
    }
    pub fn get_method(&self) -> &str {
        &self.method
    }
    pub fn get_shop(&self) -> &str {
        &self.shop
    }
    pub fn get_date(&self) -> i64 {
        self.date
    }
    pub fn get_orders(&self) -> &BTreeSet<Order> {
        &self.orders
    }
    pub fn calculate_total_price(&self) -> u32 {
        let mut acc = 0;
        for order in &self.orders {
            acc += order.calculate_price();
        }
        acc
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllPayments {
    value_set: ValueSet,
    payments: BTreeSet<Payment>,
}

impl AllPayments {
    pub fn to_json(&self, pretty: bool) -> Result<String, PaymentError> {
        if pretty {
            serde_json::to_string_pretty(self).map_err(|err| PaymentError::Generic(err.to_string()))
        } else {
            serde_json::to_string(self).map_err(|err| PaymentError::Generic(err.to_string()))
        }
    }

    pub fn from_json(json: &str) -> Result<Self, PaymentError> {
        let all_payments: AllPayments =
            serde_json::from_str(json).map_err(|err| PaymentError::Generic(err.to_string()))?;
        let missing_elements = all_payments.validate();
        if missing_elements.is_empty() {
            Ok(all_payments)
        } else {
            Err(PaymentError::MissingElements(missing_elements))
        }
    }

    fn validate(&self) -> Elements {
        let mut missing_elements = Elements::new();
        for payment in &self.payments {
            missing_elements.append(&mut payment.validate(&self.value_set));
            for order in &payment.orders {
                missing_elements.append(&mut order.validate(&self.value_set));
            }
        }
        missing_elements
    }

    pub fn new() -> Self {
        Self {
            value_set: ValueSet {
                cities: BTreeSet::new(),
                shops: BTreeSet::new(),
                methods: BTreeSet::new(),
                items: BTreeSet::new(),
            },
            payments: BTreeSet::new(),
        }
    }

    pub fn add_elements(&mut self, elements: &[Element]) -> Option<PaymentError> {
        let mut duplicates = Elements::new();
        for element in elements {
            if !match element {
                Element::City(city) => self.value_set.cities.insert(String::from(city)),
                Element::Shop(shop) => self.value_set.shops.insert(String::from(shop)),
                Element::Method(method) => self.value_set.methods.insert(String::from(method)),
                Element::Item(item) => self.value_set.items.insert(String::from(item)),
            } {
                duplicates.insert(element.clone());
            }
        }
        if duplicates.is_empty() {
            None
        } else {
            Some(PaymentError::DuplicateElements(duplicates))
        }
    }

    pub fn get_payments(&self) -> &BTreeSet<Payment> {
        &self.payments
    }
}

#[cfg(test)]
mod tests {
    use super::{AllPayments, Element, PaymentError};

    #[test]
    fn parse_invalid_json() {
        let wrong_json = r#"{"wrong_json": "12"}"#;
        let payment = AllPayments::from_json(wrong_json);
        println!("{payment:?}");
        assert!(matches!(payment, Err(PaymentError::Generic(_))));
    }

    #[test]
    fn parse_correct_json() {
        let correct_json = r#" {"payments":[{
  "city":"London","shop":"Bar","method":"Cash","date":12,
  "orders":[{"quantity":1,"unit_price":120,"item":"Apples"}]
}],
  "value_set":{
  "cities":["London"],"shops":["Bar"],"methods":["Cash"],"items":["Apples"]
}} "#;
        let payment = AllPayments::from_json(correct_json);
        println!("{payment:?}");
        assert!(payment.is_ok());
    }

    #[test]
    fn insert_values() {
        let mut payments = AllPayments::new();
        let duplicates = payments.add_elements(&[
            Element::City(String::from("City1")),
            Element::Shop(String::from("Shop1")),
            Element::Method(String::from("Method1")),
            Element::Item(String::from("Item1")),
        ]);
        assert_eq!(duplicates.len(), 0);
        let duplicates = payments.add_elements(&[
            Element::City(String::from("City1")),
            Element::Shop(String::from("Shop1")),
            Element::Method(String::from("Method1")),
            Element::Item(String::from("Item1")),
        ]);
        assert_eq!(duplicates.len(), 4);
    }
}
