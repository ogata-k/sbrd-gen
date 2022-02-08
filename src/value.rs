use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Unexpected, Visitor};

pub type DataValueMap<K> = BTreeMap<K, DataValue>;
pub type SbrdInt = i32;
pub type SbrdReal = f32;
pub type SbrdBool = bool;

pub const DATE_TIME_DEFAULT_FORMAT: &str = "%F %T";
pub const DATE_DEFAULT_FORMAT: &str = "%F";
pub const TIME_DEFAULT_FORMAT: &str = "%T";

pub(crate) fn replace_values(base_format: &str, value_map: &DataValueMap<String>) -> String {
    let mut result = String::new();
    for (i, (key, value)) in value_map.iter().enumerate() {
        let format = format!("{{{}}}", key);
        let eval_value = value.to_eval_value();
        if i == 0 {
            result = base_format.replace(&format, &eval_value);
        } else {
            result = result.replace(&format, &eval_value);
        }
    }

    result
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum DataValue {
    Int(SbrdInt),
    Real(SbrdReal),
    Bool(bool),
    String(String),
    Null,
}

struct DataValueVisitor;
impl<'de> Visitor<'de> for DataValueVisitor {
    type Value = DataValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("null or string for value parameter.")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let i = SbrdInt::try_from(v);
        match i {
            Err(_) => Err(Error::invalid_value(Unexpected::Signed(v), &self)),
            Ok(parsed) => Ok(DataValue::Int(parsed)),
        }
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let i = SbrdInt::try_from(v);
        match i {
            Err(_) => Err(Error::invalid_value(Unexpected::Unsigned(v as u64), &self)),
            Ok(parsed) => Ok(DataValue::Int(parsed)),
        }
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(DataValue::Real(v as SbrdReal))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(DataValue::String(v.to_string()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(DataValue::String(v))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(DataValue::Null)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(Self)
    }
}

impl<'de> Deserialize<'de> for DataValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(DataValueVisitor)
    }
}

impl Serialize for DataValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            DataValue::Int(v) => serializer.serialize_i64(*v as i64),
            DataValue::Real(v) => serializer.serialize_f64(*v as f64),
            DataValue::Bool(v) => serializer.serialize_bool(*v),
            DataValue::String(v) => serializer.serialize_str(v),
            DataValue::Null => serializer.serialize_unit(),
        }
    }
}

impl DataValue {
    pub fn to_eval_value(&self) -> String {
        match self {
            DataValue::Int(v) => v.to_string(),
            DataValue::Real(v) => v.to_string(),
            DataValue::Bool(v) => v.to_string(),
            DataValue::String(v) => v.to_string(),
            DataValue::Null => "null".to_string(),
        }
    }
}
