use serde::{Serialize, Deserialize, Deserializer};
use std::str::FromStr;

/// Represents a number that can be an integer or a float.
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

impl Number {
    /// Convert to u8
    pub fn as_u8(&self) -> Option<u8> {
        match self {
            Number::Int(i) if *i >= 0 && *i <= u8::MAX as i64 => Some(*i as u8),
            Number::Float(f) if *f >= 0.0 && *f <= u8::MAX as f64 => Some(*f as u8),
            _ => None,
        }
    }

    /// Convert to u16
    pub fn as_u16(&self) -> Option<u16> {
        match self {
            Number::Int(i) if *i >= 0 && *i <= u16::MAX as i64 => Some(*i as u16),
            Number::Float(f) if *f >= 0.0 && *f <= u16::MAX as f64 => Some(*f as u16),
            _ => None,
        }
    }

    /// Convert to u32
    pub fn as_u32(&self) -> Option<u32> {
        match self {
            Number::Int(i) if *i >= 0 && *i <= u32::MAX as i64 => Some(*i as u32),
            Number::Float(f) if *f >= 0.0 && *f <= u32::MAX as f64 => Some(*f as u32),
            _ => None,
        }
    }

    /// Convert to i16
    pub fn as_i16(&self) -> Option<i16> {
        match self {
            Number::Int(i) if *i >= i16::MIN as i64 && *i <= i16::MAX as i64 => Some(*i as i16),
            Number::Float(f) if *f >= i16::MIN as f64 && *f <= i16::MAX as f64 => Some(*f as i16),
            _ => None,
        }
    }

    /// Convert to i32
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Number::Int(i) if *i >= i32::MIN as i64 && *i <= i32::MAX as i64 => Some(*i as i32),
            Number::Float(f) if *f >= i32::MIN as f64 && *f <= i32::MAX as f64 => Some(*f as i32),
            _ => None,
        }
    }

    /// Convert to i64
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Number::Int(i) => Some(*i),
            Number::Float(f) if *f >= i64::MIN as f64 && *f <= i64::MAX as f64 => Some(*f as i64),
            _ => None,
        }
    }

    /// Convert to f32
    pub fn as_f32(&self) -> Option<f32> {
        match self {
            Number::Int(i) => Some(*i as f32),
            Number::Float(f) if *f <= f32::MAX.into() => Some(*f as f32),
            _ => None, // Avoid returning NaN or Infinity
        }
    }

    /// Convert to f64
    pub fn as_f64(&self) -> f64 {
        match self {
            Number::Int(i) => *i as f64,
            Number::Float(f) => *f,
        }
    }
}

/// Custom deserializer to handle Option<Number>
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
