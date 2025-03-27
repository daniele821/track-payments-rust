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

    fn get_payment(&mut self, payid: &PaymentId) -> Result<&mut PaymentDetail, PaymentError> {
        self.payments
            .get_mut(payid)
            .ok_or(PaymentError::PaymentNotFound(payid.clone()))
    }

    fn get_order(
        &mut self,
        payid: &PaymentId,
        orderid: &OrderId,
    ) -> Result<&mut OrderDetail, PaymentError> {
        self.get_payment(payid)?
            .orders
            .get_mut(orderid)
            .ok_or(PaymentError::OrderNotFound(orderid.clone()))
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
        let missing_values = orderid.get_missing_elems(&self.value_set);
        let paydetails = self.get_payment(payid)?;

        if paydetails.orders.contains_key(&orderid) {
            return Err(PaymentError::OrderDuplicated(orderid));
        }
        if !missing_values.is_empty() {
            return Err(PaymentError::MissingElements(missing_values));
        }

        assert!(paydetails.orders.insert(orderid, orderdetails).is_none());
        Ok(())
    }

    pub fn modify_payment(
        &mut self,
        payid: &PaymentId,
        paydetails: PaymentDetail,
    ) -> Result<(), PaymentError> {
        let paydetails_mut = self.get_payment(payid)?;
        *paydetails_mut = paydetails;
        Ok(())
    }

    pub fn modify_order(
        &mut self,
        payid: &PaymentId,
        orderid: &OrderId,
        orderdetails: OrderDetail,
    ) -> Result<(), PaymentError> {
        let orderdetails_mut = self.get_order(payid, orderid)?;
        *orderdetails_mut = orderdetails;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_payments_creation() {
        let orderid1 = OrderId::new(String::from("Apple"), 120);
        let orderid2 = OrderId::new(String::from("Banana"), 100);
        let orderdetails1 = OrderDetail::new(2);
        let orderdetails2 = OrderDetail::new(1);
        let payid1 = PaymentId::new(0);
        let payid2 = PaymentId::new(0);
        let paydetails = PaymentDetail::new(
            String::from("London"),
            String::from("Pub"),
            String::from("Card"),
        );
        let paydetails2 = PaymentDetail::new(
            String::from("Paris"),
            String::from("Market"),
            String::from("Cash"),
        );

        let mut values = ValueSet::new();
        values.add_values(
            vec![String::from("London"), String::from("Paris")],
            vec![String::from("Market"), String::from("Pub")],
            vec![String::from("Card"), String::from("Cash")],
            vec![String::from("Apple"), String::from("Banana")],
        );

        let mut all_payments = AllPayments::new();
        all_payments.add_values(values);

        // add payment and order
        assert_eq!(all_payments.add_payment(payid1.clone(), paydetails), Ok(()));
        assert!(
            all_payments
                .add_order(&payid1, orderid1, orderdetails1)
                .is_ok()
        );

        // test payment and order were inserted
        let order = all_payments.payments().get(&payid1);
        assert_eq!(all_payments.payments().len(), 1);
        assert_eq!(order.unwrap().orders().len(), 1);

        // modify payment and order
        let paydetails2_copy = paydetails2.clone();
        assert!(all_payments.modify_payment(&payid1, paydetails2).is_ok());
        assert_eq!(
            *all_payments.get_payment(&payid1).unwrap(),
            paydetails2_copy
        );

        // remove payment and order
    }
}
