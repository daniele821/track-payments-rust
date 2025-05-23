use super::{AllPayments, OrderDetail, OrderId, PaymentDetail, PaymentId, ValueSet};
use crate::{
    error::{Error, Result},
    time::FakeUtcTime,
    types::internment::CustomString,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValueSetJson {
    cities: BTreeSet<CustomString>,
    shops: BTreeSet<CustomString>,
    #[serde(rename = "paymentMethods")]
    methods: BTreeSet<CustomString>,
    items: BTreeSet<CustomString>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrderJson {
    item: CustomString,
    #[serde(rename = "unitPrice")]
    unit_price: u32,
    quantity: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PaymentJson {
    date: CustomString,
    city: CustomString,
    #[serde(rename = "paymentMethod")]
    method: CustomString,
    shop: CustomString,
    orders: Vec<OrderJson>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AllPaymentsJson {
    #[serde(rename = "valueSet")]
    value_set: ValueSetJson,
    payments: Vec<PaymentJson>,
}

impl AllPaymentsJson {
    pub fn from_json(json_str: &str) -> Result<Self> {
        serde_json::from_str(json_str).map_err(Error::JsonDumpFailed)
    }

    pub fn dump_json(&self, fmt: bool) -> Result<String> {
        if fmt {
            serde_json::to_string_pretty(self).map_err(Error::JsonDumpFailed)
        } else {
            serde_json::to_string(self).map_err(Error::JsonDumpFailed)
        }
    }

    pub fn from_api(self_api: &AllPayments) -> Result<Self> {
        let values = ValueSetJson {
            cities: self_api.value_set().cities().clone(),
            shops: self_api.value_set().shops().clone(),
            methods: self_api.value_set().methods().clone(),
            items: self_api.value_set().items().clone(),
        };
        let mut payments = vec![];
        for payment_api in self_api.payments() {
            let date = (*payment_api.0.date()).format_str()?;
            let city = payment_api.1.payment_details.city().clone();
            let shop = payment_api.1.payment_details.shop().clone();
            let method = payment_api.1.payment_details.method().clone();
            let mut payment = PaymentJson {
                date: date.into(),
                city,
                shop,
                method,
                orders: vec![],
            };

            let orders = payment_api.1.orders();

            for order_api in orders {
                let item = order_api.0.item().clone();
                let unit_price = *order_api.1.unit_price();
                let quantity = *order_api.1.quantity();
                let order = OrderJson {
                    item,
                    unit_price,
                    quantity,
                };

                payment.orders.push(order);
            }

            payments.push(payment);
        }

        Ok(AllPaymentsJson {
            value_set: values,
            payments,
        })
    }

    pub fn to_api(&self) -> Result<AllPayments> {
        let mut all_payments_api = AllPayments::new();
        let mut values_api = ValueSet::new();
        values_api.add_values(
            self.value_set.cities.clone(),
            self.value_set.shops.clone(),
            self.value_set.methods.clone(),
            self.value_set.items.clone(),
        );
        all_payments_api.add_values(values_api);

        for payment in &self.payments {
            let date = FakeUtcTime::parse_str(payment.date.as_str())?;
            let payid = PaymentId::new(date);
            let city = payment.city.clone();
            let shop = payment.shop.clone();
            let method = payment.method.clone();
            let paydetails = PaymentDetail::new(city, shop, method);
            all_payments_api.add_payment(payid, paydetails)?;

            for order in &payment.orders {
                let payid = PaymentId::new(date);
                let item = order.item.clone();
                let unitprice = order.unit_price;
                let quantity = order.quantity;
                let orderid = OrderId::new(item);
                let orderdetails = OrderDetail::new(unitprice, quantity);
                all_payments_api.add_order(&payid, orderid, orderdetails)?;
            }
        }
        Ok(all_payments_api)
    }
}

#[cfg(test)]
mod tests {
    use super::AllPaymentsJson;

    #[test]
    fn allpayments_legacy_json() {
        let json_string = r#"
{ "valueSet": { "cities": ["New York", "London"], "shops": ["Shop A", "Shop B"],
    "paymentMethods": ["Credit Card", "Cash"], "items": ["Apple", "Banana"] },
  "payments": [
    { "date": "2024/03/27 12:34", "city": "New York", "paymentMethod": "Credit Card", "shop": "Shop A",
      "orders": [ { "item": "Apple", "unitPrice": 100, "quantity": 2 } ] },
    { "date": "2024/03/28 09:15", "city": "London", "paymentMethod": "Cash", "shop": "Shop B",
      "orders": [ { "item": "Apple", "unitPrice": 100, "quantity": 1 },
                  { "item": "Banana", "unitPrice": 50, "quantity": 3 }
      ] } ] }
        "#;
        let all_payments = AllPaymentsJson::from_json(json_string).unwrap();
        let _ = all_payments.dump_json(false);
        let all_payments2 = AllPaymentsJson::from_json(json_string).unwrap();

        assert_eq!(all_payments, all_payments2);

        let all_payment_api = all_payments.to_api().unwrap();
        let all_payments3 = AllPaymentsJson::from_api(&all_payment_api).unwrap();

        assert_eq!(all_payments2, all_payments3);
    }
}
