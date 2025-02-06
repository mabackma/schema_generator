use serde::{Serialize, Deserialize, Deserializer};
use std::str::FromStr;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Number {
    Int(i64),
    Float(f64),
}

impl FromStr for Number {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(i) = s.parse::<i64>() {
            Ok(Number::Int(i))
        } else if let Ok(f) = s.parse::<f64>() {
            Ok(Number::Float(f))
        } else {
            Err(())
        }
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
