use super::{
    AllPayments as AllPaymentsApi, OrderDetail as OrderDetailApi, OrderId as OrderIdApi,
    PaymentDetail as PaymentDetailApi, PaymentError as PaymentErrorApi, PaymentId as PaymentIdApi,
    ValueSet as ValueSetApi,
};
use crate::time::FakeUtcTime;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const DATE_FORMAT: &str = crate::time::CUSTOM_FORMAT;

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

    pub fn from_api(self_api: &AllPaymentsApi) -> Result<Self, PaymentErrorApi> {
        Self::try_from(self_api)
    }

    pub fn to_api(&self) -> Result<AllPaymentsApi, PaymentErrorApi> {
        AllPaymentsApi::try_from(self)
    }

    pub fn convert_to_valid_legacy(&mut self) {
        for payment in &mut self.payments {
            let mut fixed_orders = BTreeMap::<String, (u32, u32)>::new();
            for order in &mut payment.orders {
                let mut final_price = order.unit_price;
                let mut final_quantity = order.quantity;
                if let Some(&(price, quant)) = fixed_orders.get(&order.item) {
                    if final_price == price {
                        final_quantity += quant;
                    } else {
                        final_price = final_price * final_quantity + price * quant;
                        final_quantity = 1;
                    }
                }
                fixed_orders.insert(order.item.clone(), (final_price, final_quantity));
            }
            payment.orders.clear();
            for (item, (unit_price, quantity)) in fixed_orders {
                payment.orders.push(Order {
                    item: item.to_string(),
                    unit_price,
                    quantity,
                });
            }
        }
    }
}

impl TryFrom<&AllPayments> for AllPaymentsApi {
    type Error = PaymentErrorApi;

    fn try_from(value: &AllPayments) -> Result<Self, Self::Error> {
        let mut all_payments_api = AllPaymentsApi::new();
        let mut values_api = ValueSetApi::new();
        values_api.add_values(
            value.value_set.cities.clone(),
            value.value_set.shops.clone(),
            value.value_set.methods.clone(),
            value.value_set.items.clone(),
        );
        all_payments_api.add_values(values_api);

        for payment in &value.payments {
            let date = FakeUtcTime::parse_str(&payment.date, DATE_FORMAT)
                .map_err(PaymentErrorApi::GenericError)?;
            let payid = PaymentIdApi::new(date);
            let city = payment.city.clone();
            let shop = payment.shop.clone();
            let method = payment.method.clone();
            let paydetails = PaymentDetailApi::new(city, shop, method);
            all_payments_api.add_payment(payid, paydetails)?;

            for order in &payment.orders {
                let payid = PaymentIdApi::new(date);
                let item = order.item.clone();
                let unitprice = order.unit_price;
                let quantity = order.quantity;
                let orderid = OrderIdApi::new(item, unitprice);
                let orderdetails = OrderDetailApi::new(quantity);
                all_payments_api.add_order(&payid, orderid, orderdetails)?;
            }
        }
        Ok(all_payments_api)
    }
}

impl TryFrom<&AllPaymentsApi> for AllPayments {
    type Error = PaymentErrorApi;

    fn try_from(value: &AllPaymentsApi) -> Result<Self, Self::Error> {
        let values = ValueSet {
            cities: value.value_set().cities().clone(),
            shops: value.value_set().shops().clone(),
            methods: value.value_set().methods().clone(),
            items: value.value_set().items().clone(),
        };
        let mut payments = vec![];
        for payment_api in value.payments() {
            let date = (*payment_api.0.date())
                .format_str(DATE_FORMAT)
                .map_err(PaymentErrorApi::GenericError)?;
            let city = payment_api.1.payment_details.city().clone();
            let shop = payment_api.1.payment_details.shop().clone();
            let method = payment_api.1.payment_details.method().clone();
            let mut payment = Payment {
                date,
                city,
                shop,
                method,
                orders: vec![],
            };

            let orders = payment_api.1.orders();

            for order_api in orders {
                let item = order_api.0.item().clone();
                let unit_price = *order_api.0.unit_price();
                let quantity = *order_api.1.quantity();
                let order = Order {
                    item,
                    unit_price,
                    quantity,
                };

                payment.orders.push(order);
            }

            payments.push(payment);
        }

        Ok(AllPayments {
            value_set: values,
            payments,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{AllPayments, AllPaymentsApi};

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
        let all_payments = AllPayments::from_json(json_string).unwrap();
        let _ = all_payments.dump_json(false);
        let all_payments2 = AllPayments::from_json(json_string).unwrap();

        assert_eq!(all_payments, all_payments2);

        let all_payment_api = AllPaymentsApi::try_from(&all_payments).unwrap();
        let all_payments3 = AllPayments::try_from(&all_payment_api).unwrap();

        assert_eq!(all_payments2, all_payments3);
    }

    #[test]
    fn allpayments_legacy_json_fixed() {
        let json_string = r#"
{ "valueSet": { "cities": ["New York", "London"], "shops": ["Shop A", "Shop B"],
    "paymentMethods": ["Credit Card", "Cash"], "items": ["Apple", "Banana"] },
  "payments": [
    { "date": "2024/03/27 12:34", "city": "New York", "paymentMethod": "Credit Card", "shop": "Shop A",
      "orders": [ { "item": "Apple", "unitPrice": 100, "quantity": 2 } ] },
    { "date": "2024/03/28 09:15", "city": "London", "paymentMethod": "Cash", "shop": "Shop B",
      "orders": [ { "item": "Apples", "unitPrice": 150, "quantity": 1 },
                  { "item": "Apples", "unitPrice": 50, "quantity": 3 },
                  { "item": "Banana", "unitPrice": 100, "quantity": 2 },
                  { "item": "Banana", "unitPrice": 100, "quantity": 1 }
      ] } ] }
        "#;
        let json_string_fixed = r#"
{ "valueSet": { "cities": ["New York", "London"], "shops": ["Shop A", "Shop B"],
    "paymentMethods": ["Credit Card", "Cash"], "items": ["Apple", "Banana"] },
  "payments": [
    { "date": "2024/03/27 12:34", "city": "New York", "paymentMethod": "Credit Card", "shop": "Shop A",
      "orders": [ { "item": "Apple", "unitPrice": 100, "quantity": 2 } ] },
    { "date": "2024/03/28 09:15", "city": "London", "paymentMethod": "Cash", "shop": "Shop B",
      "orders": [ { "item": "Apples", "unitPrice": 300, "quantity": 1 },
                  { "item": "Banana", "unitPrice": 100, "quantity": 3 }
      ] } ] }
        "#;
        let mut all_payments = AllPayments::from_json(json_string).unwrap();
        all_payments.convert_to_valid_legacy();
        let all_payments_fixed = AllPayments::from_json(json_string_fixed).unwrap();

        assert_eq!(all_payments, all_payments_fixed);
    }
}
