use internment::Intern;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CustomString {
    value: Intern<String>,
}

impl CustomString {
    pub fn new(value: String) -> Self {
        Self {
            value: Intern::new(value),
        }
    }

    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

impl From<String> for CustomString {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl Serialize for CustomString {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.value.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CustomString {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(CustomString::new(Deserialize::deserialize(deserializer)?))
    }
}

#[cfg(test)]
mod tests {
    use super::CustomString;

    #[test]
    pub fn json_conversion() {
        let json_string = r#""String1""#;
        let custom_string: CustomString = serde_json::from_str(json_string).unwrap();
        let parsed_json = serde_json::to_string(&custom_string).unwrap();
        let custom_string2: CustomString = serde_json::from_str(&parsed_json).unwrap();
        assert_eq!(custom_string, custom_string2);
    }
}
