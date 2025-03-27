#![allow(unused)]

mod json_legacy;

use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

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
}

#[derive(Serialize, Deserialize, Getters, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct AllPayments {
    value_set: ValueSet,
    payments: BTreeMap<PaymentId, PaymentDetail>,
    orders: BTreeMap<PaymentId, BTreeMap<OrderId, OrderDetail>>,
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

    pub fn check_missing_elements(&self, valid_values: &ValueSet) -> Result<(), PaymentError> {
        let mut values = ValueSet::new();
        if !valid_values.items.contains(&self.item) {
            values.add_values(vec![], vec![], vec![], vec![self.item.clone()]);
        }
        values
            .is_empty()
            .then_some(())
            .ok_or(PaymentError::MissingElements(values))
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
        Self { city, shop, method }
    }

    pub fn check_missing_elements(&self, valid_values: &ValueSet) -> Result<(), PaymentError> {
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
            .is_empty()
            .then_some(())
            .ok_or(PaymentError::MissingElements(values))
    }
}

impl AllPayments {
    pub fn new() -> Self {
        Self {
            value_set: ValueSet::new(),
            payments: BTreeMap::new(),
            orders: BTreeMap::new(),
        }
    }

    pub fn add_values(&mut self, new_values: ValueSet) {
        self.value_set.extend(new_values);
    }

    pub fn add_payment(
        &mut self,
        payid: PaymentId,
        paydetail: PaymentDetail,
    ) -> Result<(), PaymentError> {
        // checks
        if self.payments.contains_key(&payid) {
            return Err(PaymentError::PaymentDuplicated(payid));
        }
        paydetail.check_missing_elements(&self.value_set)?;

        // insert payment and empty order list
        self.orders.entry(payid.clone()).or_default();
        assert!(self.payments.insert(payid, paydetail).is_none());

        Ok(())
    }

    pub fn add_order(
        &mut self,
        payid: &PaymentId,
        orderid: OrderId,
        orderdetail: OrderDetail,
    ) -> Result<(), PaymentError> {
        // checks
        if !self.payments.contains_key(payid) {
            return Err(PaymentError::PaymentNotFound(payid.clone()));
        }
        let mut order_map = self.orders.get_mut(payid).expect("order map missing!");
        if order_map.contains_key(&orderid) {
            return Err(PaymentError::OrderDuplicated(orderid.clone()));
        }
        orderid.check_missing_elements(&self.value_set)?;

        // insert order
        assert!(order_map.insert(orderid, orderdetail).is_none());

        Ok(())
    }

    pub fn modify_payment(
        &mut self,
        payid: &PaymentId,
        paydetail: PaymentDetail,
    ) -> Result<(), PaymentError> {
        todo!()
    }

    pub fn modify_order(
        &mut self,
        payid: &PaymentId,
        orderid: OrderId,
        orderdetail: OrderDetail,
    ) -> Result<(), PaymentError> {
        todo!()
    }

    pub fn remove_payment(&mut self, payid: &PaymentId) -> Result<(), PaymentError> {
        todo!()
    }

    pub fn remove_order(
        &mut self,
        payid: &PaymentId,
        orderid: &OrderId,
    ) -> Result<(), PaymentError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_payments_creation() {
        let payid = PaymentId::new(0);
        let paydetail = PaymentDetail::new(
            String::from("London"),
            String::from("Pub"),
            String::from("Card"),
        );
        let paydetail2 = PaymentDetail::new(
            String::from("Paris"),
            String::from("Market"),
            String::from("Cash"),
        );
        let orderid = OrderId::new(String::from("Apple"), 120);
        let orderdetail = OrderDetail::new(2);
        let orderdetail2 = OrderDetail::new(3);
        let mut values = ValueSet::new();
        values.add_values(
            vec![String::from("London"), String::from("Paris")],
            vec![String::from("Market"), String::from("Pub")],
            vec![String::from("Card"), String::from("Cash")],
            vec![String::from("Apple"), String::from("Banana")],
        );

        // insert values
        let mut all_payments = AllPayments::new();
        all_payments.add_values(values);

        // insert payment
        let res = all_payments.add_payment(payid.clone(), paydetail);
        assert_eq!(res, Ok(()));

        // insert order
        let res = all_payments.add_order(&payid, orderid.clone(), orderdetail);
        assert_eq!(res, Ok(()));

        println!("INSERTED PAYMENT AND ORDER: {all_payments:#?}\n");

        // modify payment
        let res = all_payments.modify_payment(&payid, paydetail2);
        assert_eq!(res, Ok(()));

        // modify order
        let res = all_payments.modify_order(&payid, orderid.clone(), orderdetail2);
        assert_eq!(res, Ok(()));

        println!("MODIFIED PAYMENT AND ORDER: {all_payments:#?}\n");

        // remove payment
        let res = all_payments.remove_payment(&payid);
        assert_eq!(res, Ok(()));

        // remove order
        let res = all_payments.remove_order(&payid, &orderid);
        assert_eq!(res, Ok(()));

        println!("DELETED PAYMENT AND ORDER: {all_payments:#?}\n");
    }
}
