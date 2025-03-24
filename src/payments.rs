use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Element {
    City(String),
    Shop(String),
    Method(String),
    Item(String),
}

pub type MissingElements = HashSet<Element>;

#[derive(Debug)]
pub enum PaymentError {
    JsonParseError(serde_json::Error),
    JsonDumpError(serde_json::Error),
    MissingElements(HashSet<Element>),
}

#[derive(Debug, Serialize, Deserialize)]
struct ValueSet {
    cities: HashSet<String>,
    shops: HashSet<String>,
    methods: HashSet<String>,
    items: HashSet<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Order {
    unit_price: u32,
    quantity: u32,
    item: String,
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item && self.unit_price == other.unit_price
    }
}
impl Eq for Order {}
impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Order {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.item.cmp(&other.item) {
            std::cmp::Ordering::Equal => self.unit_price.cmp(&other.unit_price),
            item_order => item_order,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Payment {
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
        Some(self.cmp(other))
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
    pub fn to_json(&self) -> Result<String, PaymentError> {
        serde_json::to_string(self).map_err(PaymentError::JsonDumpError)
    }

    pub fn from_json(json: &str) -> Result<Self, PaymentError> {
        let all_payments: AllPayments =
            serde_json::from_str(json).map_err(PaymentError::JsonParseError)?;
        let missing_elements = all_payments.validate();
        if missing_elements.is_empty() {
            Ok(all_payments)
        } else {
            Err(PaymentError::MissingElements(missing_elements))
        }
    }

    fn validate(&self) -> MissingElements {
        let mut missing_elements = MissingElements::new();
        for payment in &self.payments {
            let city = &payment.city;
            let shop = &payment.shop;
            let method = &payment.method;
            if !self.value_set.cities.contains(city) {
                missing_elements.insert(Element::City(city.clone()));
            }
            if !self.value_set.shops.contains(shop) {
                missing_elements.insert(Element::Shop(shop.clone()));
            }
            if !self.value_set.methods.contains(method) {
                missing_elements.insert(Element::Method(method.clone()));
            }
            for order in &payment.orders {
                let item = &order.item;
                if !self.value_set.items.contains(item) {
                    missing_elements.insert(Element::Item(item.clone()));
                }
            }
        }
        missing_elements
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn parse_invalid_json() {
        let wrong_json = r#"{"wrong_json": "12"}"#;
        let payment = super::AllPayments::from_json(wrong_json);
        println!("{payment:?}");
        assert!(matches!(
            payment,
            Err(super::PaymentError::JsonParseError(_))
        ));
    }

    #[test]
    fn parse_correct_json() {
        let correct_json = r#"
{ "payments": [
    {
      "city": "London",
      "shop": "Bar",
      "method": "Cash",
      "date": 12,
      "orders": [ {
          "quantity": 1,
          "unit_price": 120,
          "item": "Apples"
        }
      ]
    }
  ],
  "value_set": {
    "cities": [ "London" ],
    "shops": [ "Bar" ],
    "methods": [ "Cash" ],
    "items": [ "Apples" ]
  }
}
        "#;
        let payment = super::AllPayments::from_json(correct_json);
        println!("{payment:?}");
        assert!(payment.is_ok());
    }
}
