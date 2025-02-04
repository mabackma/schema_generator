use serde::{Serialize, Deserialize, Deserializer};
use serde::de::{self, Visitor};
use std::fmt;
use std::str::FromStr;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Number {
    Int(i64),
    Float(f64),
}

impl<'de> Deserialize<'de> for Number {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NumberVisitor;

        impl<'de> Visitor<'de> for NumberVisitor {
            type Value = Number;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a number that can be an integer or a float")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let trimmed = value.trim();
                
                if let Ok(int_value) = i64::from_str(trimmed) {
                    Ok(Number::Int(int_value))
                } else if let Ok(float_value) = f64::from_str(trimmed) {
                    Ok(Number::Float(float_value))
                } else {
                    Err(E::custom(format!("invalid number format: '{}'", value)))
                }
            }
        }

        deserializer.deserialize_str(NumberVisitor)
    }
}

// Custom deserializer to handle Option<Number>
pub fn deserialize_optional_number<'de, D>(deserializer: D) -> Result<Option<Number>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s.as_deref().map(str::trim) {
        Some("") | None => Ok(None), // Return None for empty fields
        Some(value) => {
            if let Ok(int_value) = value.parse::<i64>() {
                Ok(Some(Number::Int(int_value)))
            } else if let Ok(float_value) = value.parse::<f64>() {
                Ok(Some(Number::Float(float_value)))
            } else {
                Err(serde::de::Error::custom(format!("invalid number format: '{}'", value)))
            }
        }
    }
}
