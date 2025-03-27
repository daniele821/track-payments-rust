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

    pub fn get_missing_elems(&self, valid_values: &ValueSet) -> Result<(), PaymentError> {
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

    pub fn get_missing_elems(&self, valid_values: &ValueSet) -> Result<(), PaymentError> {
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
        let i = self
            .orders
            .get_mut(payid)
            .ok_or(PaymentError::PaymentNotFound(payid.clone()))?;
        todo!()
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
        let payid = PaymentId::new(0);
        let paydetails1 = PaymentDetail::new(
            String::from("London"),
            String::from("Pub"),
            String::from("Card"),
        );
        let paydetails2 = PaymentDetail::new(
            String::from("Paris"),
            String::from("Market"),
            String::from("Cash"),
        );
        let orderid = OrderId::new(String::from("Apple"), 120);
        let orderdetails1 = OrderDetail::new(2);
        let orderdetails2 = OrderDetail::new(1);

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
        assert_eq!(all_payments.add_payment(payid.clone(), paydetails1), Ok(()));
        assert!(
            all_payments
                .add_order(&payid, orderid.clone(), orderdetails1)
                .is_ok()
        );

        // test payment and order were inserted
        let order = all_payments.payments().get(&payid);
        assert_eq!(all_payments.payments().len(), 1);
        assert_eq!(order.unwrap().orders().len(), 1);

        // modify payment and order
        assert!(
            all_payments
                .modify_payment(&payid, paydetails2.clone())
                .is_ok()
        );
        assert_eq!(*all_payments.get_payment(&payid).unwrap(), paydetails2);
        assert!(
            all_payments
                .modify_order(&payid, &orderid, orderdetails2.clone())
                .is_ok()
        );
        assert_eq!(
            *all_payments.get_order(&payid, &orderid).unwrap(),
            orderdetails2
        );

        // remove payment and order
    }
}
