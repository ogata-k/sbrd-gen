use std::fmt;

use chrono::naive::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Unexpected, Visitor};

pub type SbrdInt = i32;
pub type SbrdReal = f32;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum DataValue {
    Int(SbrdInt),
    Real(SbrdReal),
    Bool(bool),
    String(String),
    DateTime(NaiveDateTime),
    Date(NaiveDate),
    Time(NaiveTime),
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
            // need 0 padding
            DataValue::DateTime(v) => serializer.serialize_str(&v.format("%F %T").to_string()),
            // need 0 padding
            DataValue::Date(v) => serializer.serialize_str(&v.format("%F").to_string()),
            // need 0 padding
            DataValue::Time(v) => serializer.serialize_str(&v.format("%T").to_string()),
            DataValue::Null => serializer.serialize_unit(),
        }
    }
}
