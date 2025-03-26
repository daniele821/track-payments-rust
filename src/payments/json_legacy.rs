#![allow(unused)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValueSet {
    cities: BTreeSet<String>,
    shops: BTreeSet<String>,
    #[serde(rename = "paymentMethods")]
    methods: BTreeSet<String>,
    items: BTreeSet<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Order {
    item: String,
    #[serde(rename = "unitPrice")]
    unit_price: u32,
    quantity: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Payment {
    date: String,
    city: String,
    #[serde(rename = "paymentMethod")]
    method: String,
    shop: String,
    orders: Vec<Order>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AllPayments {
    #[serde(rename = "valueSet")]
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
    fn allpayments_legacy_json() {
        let json_string = r#"
{
  "valueSet": {
    "cities": ["New York", "London"],
    "shops": ["Shop A", "Shop B"],
    "paymentMethods": ["Credit Card", "Cash"],
    "items": ["Apple", "Banana"]
  },
  "payments": [
    {
      "date": "2024/03/27 12:34",
      "city": "New York",
      "paymentMethod": "Credit Card",
      "shop": "Shop A",
      "orders": [
        {
          "item": "Apple",
          "unitPrice": 100,
          "quantity": 2
        }
      ]
    },
    {
      "date": "2024/03/28 09:15",
      "city": "London",
      "paymentMethod": "Cash",
      "shop": "Shop B",
      "orders": [
        {
          "item": "Banana",
          "unitPrice": 50,
          "quantity": 3
        },
        {
          "item": "Apple",
          "unitPrice": 100,
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
