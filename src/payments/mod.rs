mod json;
mod json_;

use crate::{
    error::{Error, Result},
    time::FakeUtcTime,
};
use derive_getters::Getters;
use json_::AllPaymentsJson;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Getters, Debug, PartialEq, Eq, Clone, Default)]
pub struct ValueSet {
    cities: BTreeSet<String>,
    shops: BTreeSet<String>,
    methods: BTreeSet<String>,
    items: BTreeSet<String>,
}

#[derive(Getters, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct OrderId {
    item: String,
}

#[derive(Getters, Debug, PartialEq, Eq, Clone)]
pub struct OrderDetail {
    unit_price: u32,
    quantity: u32,
}

#[derive(Getters, Debug, PartialEq, Eq, Clone)]
pub struct PayOrdersDetail {
    payment_details: PaymentDetail,
    orders: BTreeMap<OrderId, OrderDetail>,
}

#[derive(Getters, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PaymentId {
    date: FakeUtcTime,
}

#[derive(Getters, Debug, PartialEq, Eq, Clone)]
pub struct PaymentDetail {
    city: String,
    shop: String,
    method: String,
}

#[derive(Getters, Debug, PartialEq, Eq, Clone, Default)]
pub struct AllPayments {
    value_set: ValueSet,
    payments: BTreeMap<PaymentId, PayOrdersDetail>,
}

impl ValueSet {
    pub fn new() -> Self {
        Self::default()
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
    pub fn new(item: String) -> Self {
        Self { item }
    }

    pub fn check_missing_elements(&self, valid_values: &ValueSet) -> Result<()> {
        let mut values = ValueSet::new();
        if !valid_values.items.contains(&self.item) {
            values.add_values(vec![], vec![], vec![], vec![self.item.clone()]);
        }
        values
            .is_empty()
            .then_some(())
            .ok_or(values)
            .map_err(Error::MissingElements)
    }
}

impl From<String> for OrderId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl OrderDetail {
    pub fn new(unit_price: u32, quantity: u32) -> Self {
        Self {
            unit_price,
            quantity,
        }
    }
}

impl PayOrdersDetail {
    pub fn new(payment_details: PaymentDetail) -> Self {
        Self {
            payment_details,
            orders: BTreeMap::new(),
        }
    }

    pub fn calcualte_total_price(&self) -> u32 {
        self.orders
            .values()
            .map(|det| det.unit_price * det.quantity)
            .sum()
    }
}

impl From<PaymentDetail> for PayOrdersDetail {
    fn from(value: PaymentDetail) -> Self {
        Self::new(value)
    }
}

impl PaymentId {
    pub fn new(date: FakeUtcTime) -> Self {
        Self { date }
    }
}

impl From<FakeUtcTime> for PaymentId {
    fn from(value: FakeUtcTime) -> Self {
        Self::new(value)
    }
}

impl PaymentDetail {
    pub fn new(city: String, shop: String, method: String) -> Self {
        Self { city, shop, method }
    }

    pub fn check_missing_elements(&self, valid_values: &ValueSet) -> Result<()> {
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
            .ok_or(values)
            .map_err(Error::MissingElements)
    }
}

impl AllPayments {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_json(json_str: &str) -> Result<Self> {
        AllPaymentsJson::from_json(json_str)?.to_api()
    }

    pub fn to_json(&self, fmt: bool) -> Result<String> {
        AllPaymentsJson::from_api(self)?.dump_json(fmt)
    }

    pub fn add_values(&mut self, new_values: ValueSet) {
        self.value_set.extend(new_values);
    }

    pub fn add_payment(&mut self, payid: PaymentId, paydetail: PaymentDetail) -> Result<()> {
        if self.payments.contains_key(&payid) {
            return Err(Error::PaymentDuplicated(payid));
        }
        paydetail.check_missing_elements(&self.value_set)?;

        assert!(self.payments.insert(payid, paydetail.into()).is_none());

        Ok(())
    }

    pub fn add_order(
        &mut self,
        payid: &PaymentId,
        orderid: OrderId,
        orderdetail: OrderDetail,
    ) -> Result<()> {
        let order_map = &mut self
            .payments
            .get_mut(payid)
            .ok_or_else(|| Error::PaymentNotFound(payid.clone()))?
            .orders;

        if order_map.contains_key(&orderid) {
            return Err(Error::OrderDuplicated(payid.clone(), orderid.clone()));
        }
        orderid.check_missing_elements(&self.value_set)?;

        assert!(order_map.insert(orderid, orderdetail).is_none());

        Ok(())
    }

    pub fn modify_payment(&mut self, payid: &PaymentId, paydetail: PaymentDetail) -> Result<()> {
        let paydetail_mut = self
            .payments
            .get_mut(payid)
            .ok_or_else(|| Error::PaymentNotFound(payid.clone()))?;
        paydetail.check_missing_elements(&self.value_set)?;

        paydetail_mut.payment_details = paydetail;

        Ok(())
    }

    pub fn modify_order(
        &mut self,
        payid: &PaymentId,
        orderid: &OrderId,
        orderdetail: OrderDetail,
    ) -> Result<()> {
        let order_map = &mut self
            .payments
            .get_mut(payid)
            .ok_or_else(|| Error::PaymentNotFound(payid.clone()))?
            .orders;
        let orderdetail_mut = order_map
            .get_mut(orderid)
            .ok_or_else(|| Error::OrderNotFound(payid.clone(), orderid.clone()))?;
        orderid.check_missing_elements(&self.value_set)?;

        *orderdetail_mut = orderdetail;

        Ok(())
    }

    pub fn remove_payment(&mut self, payid: &PaymentId) -> Result<()> {
        self.payments
            .remove(payid)
            .map(|_| {})
            .ok_or_else(|| Error::PaymentNotFound(payid.clone()))
    }

    pub fn remove_order(&mut self, payid: &PaymentId, orderid: &OrderId) -> Result<()> {
        let order_map = &mut self
            .payments
            .get_mut(payid)
            .ok_or_else(|| Error::PaymentNotFound(payid.clone()))?
            .orders;
        order_map
            .remove(orderid)
            .map(|_| ())
            .ok_or_else(|| Error::OrderNotFound(payid.clone(), orderid.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::{AllPayments, OrderDetail, OrderId, PaymentDetail, PaymentId, ValueSet};

    #[test]
    fn all_payments_creation() {
        let payid = PaymentId::new(0.into());
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
        let orderid = OrderId::new(String::from("Apple"));
        let orderdetail = OrderDetail::new(120, 2);
        let orderdetail2 = OrderDetail::new(120, 3);
        let mut values = ValueSet::new();
        values.add_values(
            vec![String::from("London"), String::from("Paris")],
            vec![String::from("Market"), String::from("Pub")],
            vec![String::from("Card"), String::from("Cash")],
            vec![String::from("Apple"), String::from("Banana")],
        );

        // insert values
        let mut all_payments = AllPayments::new();
        all_payments.add_values(values.clone());
        assert_eq!(all_payments.value_set(), &values);
        println!("INSERT VALUES: {all_payments:?}");

        // insert payment
        let res = all_payments.add_payment(payid.clone(), paydetail.clone());
        assert_eq!(res, Ok(()));
        let newval = all_payments.payments().first_key_value().unwrap().1;
        assert_eq!(newval.payment_details(), &paydetail);
        println!("INSERT PAYMENT: {all_payments:?}");

        // insert order
        let res = all_payments.add_order(&payid, orderid.clone(), orderdetail.clone());
        assert_eq!(res, Ok(()));
        let newval = all_payments.payments().first_key_value().unwrap().1;
        let newval = newval.orders().first_key_value().unwrap().1;
        assert_eq!(newval, &orderdetail);
        println!("INSERT ORDER: {all_payments:?}");

        // json conversion
        let json = all_payments.to_json(false).unwrap();
        let all_payments2 = AllPayments::from_json(&json).unwrap();
        assert_eq!(all_payments, all_payments2);
        println!("JSON: {json}");

        // modify payment
        let res = all_payments.modify_payment(&payid, paydetail2.clone());
        assert_eq!(res, Ok(()));
        let newval = all_payments.payments.first_key_value().unwrap().1;
        assert_eq!(newval.payment_details(), &paydetail2);
        println!("MODIFIED PAYMENT: {all_payments:?}");

        // modify order
        let res = all_payments.modify_order(&payid, &orderid, orderdetail2.clone());
        assert_eq!(res, Ok(()));
        let newval = all_payments.payments().first_key_value().unwrap().1;
        let newval = newval.orders().first_key_value().unwrap().1;
        assert_eq!(newval, &orderdetail2);
        println!("MODIFIED ORDER: {all_payments:?}");

        // remove order
        let res = all_payments.remove_order(&payid, &orderid);
        assert_eq!(res, Ok(()));
        let pay1 = all_payments.payments().first_key_value().unwrap();
        assert_eq!(pay1.1.orders().len(), 0);
        println!("REMOVED ORDER: {all_payments:?}");

        // remove payment
        let res = all_payments.remove_payment(&payid);
        assert_eq!(res, Ok(()));
        assert_eq!(all_payments.payments.len(), 0);
        println!("REMOVED PAYMENT: {all_payments:?}");
    }
}
