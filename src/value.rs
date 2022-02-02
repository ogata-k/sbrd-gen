use chrono::naive::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Serialize, Serializer};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum DataValue {
    Int(i16),
    Real(f32),
    Bool(bool),
    String(String),
    DateTime(NaiveDateTime),
    Date(NaiveDate),
    Time(NaiveTime),
    Null,
}

impl Serialize for DataValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            DataValue::Int(v) => serializer.serialize_i16(*v),
            DataValue::Real(v) => serializer.serialize_f32(*v),
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
