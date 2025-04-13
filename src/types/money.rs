use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Money {
    value: u32,
}

impl Money {
    pub fn new(value_in_cents: u32) -> Self {
        Self {
            value: value_in_cents,
        }
    }

    pub fn new_cents(value: u32, cents: u8) -> Self {
        Self::new(value * 100 + cents as u32)
    }

    pub fn cents(&self) -> u32 {
        self.value
    }

    pub fn value_dec(&self) -> f64 {
        self.value as f64
    }
}

impl From<u32> for Money {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl Serialize for Money {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.value.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Money {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Money::new(Deserialize::deserialize(deserializer)?))
    }
}

#[cfg(test)]
mod tests {
    use super::Money;

    #[test]
    pub fn json_conversion() {
        let json_string = "34";
        let custom_money: Money = serde_json::from_str(json_string).unwrap();
        let parsed_json = serde_json::to_string(&custom_money).unwrap();
        let custom_money2: Money = serde_json::from_str(&parsed_json).unwrap();
        assert_eq!(custom_money, custom_money2);
    }
}
