#![allow(unused)]

use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

mod json_default;
mod json_legacy;

pub use json_default::AllPayments as AllPaymentsDefault;
pub use json_legacy::AllPayments as AllPaymentsLegacy;

#[derive(Serialize, Deserialize, Getters, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ValueSet {
    cities: BTreeSet<String>,
    shops: BTreeSet<String>,
    methods: BTreeSet<String>,
    items: BTreeSet<String>,
}

#[derive(Serialize, Deserialize, Getters, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrderId {
    item: String,
    unit_price: u32,
}
#[derive(Serialize, Deserialize, Getters, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrderDetail {
    quantity: u32,
}

#[derive(Serialize, Deserialize, Getters, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PaymentId {
    date: i64,
}
#[derive(Serialize, Deserialize, Getters, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PaymentDetail {
    city: String,
    shop: String,
    method: String,
    orders: BTreeMap<OrderId, OrderDetail>,
}

#[derive(Serialize, Deserialize, Getters, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AllPayments {
    value_set: ValueSet,
    payments: BTreeMap<PaymentId, PaymentDetail>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PaymentError {
    OrderDuplicated,
    OrderNotFound,
    PaymentDuplicated,
    PaymentNotFound,
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

    pub fn add_values<Iter: IntoIterator<Item = String>>(
        &mut self,
        cities: Iter,
        shops: Iter,
        methods: Iter,
        items: Iter,
    ) {
        self.cities.extend(cities);
        self.shops.extend(shops);
        self.methods.extend(methods);
        self.items.extend(items);
    }

    pub fn extend(&mut self, other: ValueSet) {
        self.add_values(other.cities, other.shops, other.methods, other.items);
    }

    pub fn intersection(&self, other: &ValueSet) -> ValueSet {
        let mut intersection = ValueSet::new();
        let cities_intersection = self.cities.intersection(&other.cities);
        let shops_intersection = self.shops.intersection(&other.shops);
        let methods_intersection = self.methods.intersection(&other.methods);
        let items_intersection = self.items.intersection(&other.items);
        intersection.cities.extend(cities_intersection.cloned());
        intersection.shops.extend(shops_intersection.cloned());
        intersection.methods.extend(methods_intersection.cloned());
        intersection.items.extend(items_intersection.cloned());
        intersection
    }

    pub fn is_empty(&self) -> bool {
        self.cities.is_empty()
            && self.shops.is_empty()
            && self.methods.is_empty()
            && self.items.is_empty()
    }
}

impl OrderId {
    pub fn new(item: String, unit_price: u32) -> Self {
        Self { item, unit_price }
    }
}

impl OrderDetail {
    pub fn new(quantity: u32) -> Self {
        Self { quantity }
    }
}

impl PaymentId {
    pub fn new(date: i64) -> Self {
        Self { date }
    }
}

impl PaymentDetail {
    pub fn new(city: String, shop: String, method: String) -> Self {
        Self {
            city,
            shop,
            method,
            orders: BTreeMap::new(),
        }
    }

    pub fn total_price(&self) -> u32 {
        let mut acc = 0;
        for order in &self.orders {
            acc += order.0.unit_price * order.1.quantity;
        }
        acc
    }
}

impl AllPayments {
    pub fn new() -> Self {
        Self {
            value_set: ValueSet::new(),
            payments: BTreeMap::new(),
        }
    }

    pub fn get_missing_values(&self) -> ValueSet {
        let values = &self.value_set;
        let mut missing_values = ValueSet::new();
        for payment in &self.payments {
            if !values.cities.contains(&payment.1.city) {
                missing_values.cities.insert(payment.1.city.clone());
            }
            if !values.shops.contains(&payment.1.shop) {
                missing_values.shops.insert(payment.1.shop.clone());
            }
            if !values.methods.contains(&payment.1.method) {
                missing_values.methods.insert(payment.1.method.clone());
            }
            for order in &payment.1.orders {
                if !values.items.contains(&order.0.item) {
                    missing_values.items.insert(order.0.item.clone());
                }
            }
        }
        missing_values
    }

    pub fn add_values(&mut self, new_values: ValueSet) {
        self.value_set.extend(new_values);
    }

    pub fn add_payment(
        &mut self,
        payid: PaymentId,
        paydetails: PaymentDetail,
    ) -> Result<(), PaymentError> {
        if self.payments.contains_key(&payid) {
            return Err(PaymentError::PaymentDuplicated);
        }
        assert!(self.payments.insert(payid, paydetails).is_none());
        Ok(())
    }

    pub fn add_order(
        &mut self,
        payid: &PaymentId,
        orderid: OrderId,
        orderdetails: OrderDetail,
    ) -> Result<(), PaymentError> {
        let paydetails = self
            .payments
            .get_mut(payid)
            .ok_or(PaymentError::PaymentNotFound)?;
        if paydetails.orders.contains_key(&orderid) {
            return Err(PaymentError::OrderDuplicated);
        }
        assert!(paydetails.orders.insert(orderid, orderdetails).is_none());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_payments_creation() {
        let orderid = OrderId::new(String::from("Apple"), 120);
        let orderdetails = OrderDetail::new(2);
        let payid = PaymentId::new(0);
        let payid_copy = PaymentId::new(0);
        let paydetails = PaymentDetail::new(
            String::from("London"),
            String::from("Market"),
            String::from("Card"),
        );
        let mut values = ValueSet::new();
        values.add_values(
            vec![String::from("London")],
            vec![String::from("Market")],
            vec![String::from("Card")],
            vec![String::from("Apple")],
        );

        // add payment and order
        let mut all_payments1 = AllPayments::new();
        all_payments1.add_values(values);
        assert_eq!(all_payments1.add_payment(payid, paydetails), Ok(()));
        let res = all_payments1.add_order(&payid_copy, orderid, orderdetails);
        assert_eq!(res, Ok(()));

        // test payment and order were inserted
        let order = all_payments1.payments().get(&payid_copy);
        assert_eq!(all_payments1.payments().len(), 1);
        assert_eq!(order.unwrap().orders().len(), 1);

        // test missing values funcion works
        assert!(all_payments1.get_missing_values().is_empty());
    }
}
