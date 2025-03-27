#![allow(unused)]

use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

mod json_default;
mod json_legacy;

pub use json_default::AllPayments as AllPaymentsDefault;
pub use json_legacy::AllPayments as AllPaymentsLegacy;

#[derive(Serialize, Deserialize, Getters, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ValueSet {
    cities: BTreeSet<String>,
    shops: BTreeSet<String>,
    methods: BTreeSet<String>,
    items: BTreeSet<String>,
}

#[derive(Serialize, Deserialize, Getters, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct OrderId {
    item: String,
    unit_price: u32,
}
#[derive(Serialize, Deserialize, Getters, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct OrderDetail {
    quantity: u32,
}

#[derive(Serialize, Deserialize, Getters, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PaymentId {
    date: i64,
}
#[derive(Serialize, Deserialize, Getters, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PaymentDetail {
    city: String,
    shop: String,
    method: String,
    orders: BTreeMap<OrderId, OrderDetail>,
}

#[derive(Serialize, Deserialize, Getters, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct AllPayments {
    value_set: ValueSet,
    payments: BTreeMap<PaymentId, PaymentDetail>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum PaymentError {
    OrderDuplicated(OrderId),
    OrderNotFound(OrderId),
    PaymentDuplicated(PaymentId),
    PaymentNotFound(PaymentId),
    MissingElements(ValueSet),
    GenericError(String),
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

    pub fn get_missing_elems(&self, valid_values: &ValueSet) -> ValueSet {
        let mut values = ValueSet::new();
        if !valid_values.items.contains(&self.item) {
            values.add_values(vec![], vec![], vec![], vec![self.item.clone()]);
        }
        values
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

    pub fn get_missing_elems(&self, valid_values: &ValueSet) -> ValueSet {
        let mut values = ValueSet::new();
        if !valid_values.cities.contains(&self.city) {
            values.add_values(vec![self.city.clone()], vec![], vec![], vec![]);
        }
        if !valid_values.shops.contains(&self.shop) {
            values.add_values(vec![], vec![self.shop.clone()], vec![], vec![]);
        }
        if !valid_values.methods.contains(&self.method) {
            values.add_values(vec![], vec![], vec![self.method.clone()], vec![]);
        }
        values
    }
}

impl AllPayments {
    pub fn new() -> Self {
        Self {
            value_set: ValueSet::new(),
            payments: BTreeMap::new(),
        }
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
            return Err(PaymentError::PaymentDuplicated(payid));
        }
        let missing_values = paydetails.get_missing_elems(&self.value_set);
        if !missing_values.is_empty() {
            return Err(PaymentError::MissingElements(missing_values));
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
            .ok_or(PaymentError::PaymentNotFound(payid.clone()))?;
        if paydetails.orders.contains_key(&orderid) {
            return Err(PaymentError::OrderDuplicated(orderid));
        }
        let missing_values = orderid.get_missing_elems(&self.value_set);
        if !missing_values.is_empty() {
            return Err(PaymentError::MissingElements(missing_values));
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
    }
}
