//! WIP: this module is still work in progress, and very unstable!

use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use serde::{Serialize, ser::SerializeStruct};
use std::fmt;

use super::ValueSet;

impl Serialize for ValueSet {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("valueSet", 4)?;
        state.serialize_field("cities", &self.cities)?;
        state.serialize_field("shops", &self.shops)?;
        state.serialize_field("paymentMethods", &self.methods)?;
        state.serialize_field("items", &self.items)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for ValueSet {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // Define the expected fields
        const FIELDS: &[&str] = &["cities", "shops", "paymentMethods", "items"];

        // Custom visitor to handle the struct
        struct ValueSetVisitor;

        impl<'de> Visitor<'de> for ValueSetVisitor {
            type Value = ValueSet;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ValueSet")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ValueSet, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut cities = None;
                let mut shops = None;
                let mut methods = None;
                let mut items = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "cities" => {
                            if cities.is_some() {
                                return Err(de::Error::duplicate_field("cities"));
                            }
                            cities = Some(map.next_value()?);
                        }
                        "shops" => {
                            if shops.is_some() {
                                return Err(de::Error::duplicate_field("shops"));
                            }
                            shops = Some(map.next_value()?);
                        }
                        "paymentMethods" => {
                            if methods.is_some() {
                                return Err(de::Error::duplicate_field("paymentMethods"));
                            }
                            methods = Some(map.next_value()?);
                        }
                        "items" => {
                            if items.is_some() {
                                return Err(de::Error::duplicate_field("items"));
                            }
                            items = Some(map.next_value()?);
                        }
                        _ => {
                            // Ignore unknown fields
                            let _: de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                Ok(ValueSet {
                    cities: cities.unwrap_or_default(),
                    shops: shops.unwrap_or_default(),
                    methods: methods.unwrap_or_default(),
                    items: items.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct("ValueSet", FIELDS, ValueSetVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use std::collections::BTreeSet;

    #[test]
    fn test_valueset_serialization() {
        let value_set = ValueSet {
            cities: BTreeSet::from(["New York".to_string(), "London".to_string()]),
            shops: BTreeSet::from(["Shop A".to_string(), "Shop B".to_string()]),
            methods: BTreeSet::from(["Credit Card".to_string(), "PayPal".to_string()]),
            items: BTreeSet::from(["Item 1".to_string(), "Item 2".to_string()]),
        };

        let serialized = serde_json::to_string(&value_set).unwrap();
        println!("{serialized}");

        // Check the JSON contains all fields
        assert!(serialized.contains("\"cities\""));
        assert!(serialized.contains("\"shops\""));
        assert!(serialized.contains("\"paymentMethods\""));
        assert!(serialized.contains("\"items\""));

        // Check some sample values
        assert!(serialized.contains("New York"));
        assert!(serialized.contains("London"));
        assert!(serialized.contains("Shop A"));
        assert!(serialized.contains("Credit Card"));
        assert!(serialized.contains("Item 1"));

        // Verify the structure by deserializing
        let deserialized: serde_json::Value = serde_json::from_str(&serialized).unwrap();
        assert!(deserialized["cities"].is_array());
        assert!(deserialized["shops"].is_array());
        assert!(deserialized["paymentMethods"].is_array());
        assert!(deserialized["items"].is_array());

        // Check array lengths
        assert_eq!(deserialized["cities"].as_array().unwrap().len(), 2);
        assert_eq!(deserialized["shops"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_deserialization() {
        let json = r#"
        {
            "cities": ["New York", "London"],
            "shops": ["Shop A"],
            "paymentMethods": ["Credit Card"],
            "items": ["Item 1", "Item 2"]
        }
    "#;

        let value_set: ValueSet = serde_json::from_str(json).unwrap();

        assert_eq!(
            value_set.cities,
            BTreeSet::from(["New York".into(), "London".into()])
        );
        assert_eq!(value_set.shops, BTreeSet::from(["Shop A".into()]));
        assert_eq!(value_set.methods, BTreeSet::from(["Credit Card".into()]));
        assert_eq!(
            value_set.items,
            BTreeSet::from(["Item 1".into(), "Item 2".into()])
        );
    }
}
