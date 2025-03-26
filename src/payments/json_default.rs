#![allow(unused)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValueSet {
    cities: BTreeSet<String>,
    shops: BTreeSet<String>,
    methods: BTreeSet<String>,
    items: BTreeSet<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Order {
    item: String,
    unit_price: u32,
    quantity: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Payment {
    date: i64,
    city: String,
    method: String,
    shop: String,
    orders: Vec<Order>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AllPayments {
    value_set: ValueSet,
    payments: Vec<Payment>,
}

impl AllPayments {
    pub fn from_json(json_str: &str) -> Result<Self, String> {
        serde_json::from_str(json_str).map_err(|err| err.to_string())
    }

    pub fn dump_json(&self, fmt: bool) -> Result<String, String> {
        if fmt {
            serde_json::to_string_pretty(self).map_err(|err| err.to_string())
        } else {
            serde_json::to_string(self).map_err(|err| err.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AllPayments;

    #[test]
    fn allpayments_default_json() {
        let json_string = r#"
{
  "value_set": {
    "cities": ["New York", "London"],
    "shops": ["Shop A", "Shop B"],
    "methods": ["Credit", "Cash"],
    "items": ["Apple", "Banana"]
  },
  "payments": [
    {
      "date": 1672531200,
      "city": "New York",
      "method": "Credit",
      "shop": "Shop A",
      "orders": [
        {
          "item": "Apple",
          "unit_price": 100,
          "quantity": 2
        }
      ]
    },
    {
      "date": 1672617600,
      "city": "London",
      "method": "Cash",
      "shop": "Shop B",
      "orders": [
        {
          "item": "Banana",
          "unit_price": 50,
          "quantity": 3
        },
        {
          "item": "Apple",
          "unit_price": 100,
          "quantity": 1
        }
      ]
    }
  ]
}
        "#;
        let all_payments = AllPayments::from_json(json_string).unwrap();
        let parsed_json = all_payments.dump_json(false);
        let all_payments2 = AllPayments::from_json(json_string).unwrap();

        assert_eq!(all_payments, all_payments2);
    }
}
