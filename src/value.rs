use serde::{Serialize, Serializer};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum DataValue {
    Int(i16),
    Real(f32),
    Bool(bool),
    String(String),
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
            DataValue::Null => serializer.serialize_unit(),
        }
    }
}
