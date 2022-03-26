#![deny(missing_debug_implementations)]
//! Module for value

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use rt_format::{Format, FormatArgument, NoNamedArguments, ParsedFormat, Specifier};
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Debug, Formatter};

use serde::de::{Error, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// KVS for this crate
pub type ValueMap<K, S> = BTreeMap<K, S>;
/// KVS for [`DataValue`]
///
/// [`DataValue`]: ./enum.DataValue.html
pub type DataValueMap<K> = ValueMap<K, DataValue>;

/// Integer type for this crate
pub type SbrdInt = i32;
/// Real type for this crate
pub type SbrdReal = f32;
/// Boolean type for this crate
pub type SbrdBool = bool;
/// String type for this crate
pub type SbrdString = String;
/// DateTime type for this crate
pub type SbrdDateTime = NaiveDateTime;
/// Date type for this crate
pub type SbrdDate = NaiveDate;
/// Time type for this crate
pub type SbrdTime = NaiveTime;

/// Default format string for [`SbrdDateTime`]
///
/// [`SbrdDateTime`]: ./type.SbrdDateTime.html
pub const DATE_TIME_DEFAULT_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
/// Default format string for [`SbrdDate`]
///
/// [`SbrdDate`]: ./type.SbrdDate.html
pub const DATE_DEFAULT_FORMAT: &str = "%Y-%m-%d";
/// Default format string for [`SbrdTime`]
///
/// [`SbrdTime`]: ./type.SbrdTime.html
pub const TIME_DEFAULT_FORMAT: &str = "%H:%M:%S";

/// Value for [`Schema`]
///
/// [`Schema`]: ../schema/struct.Schema.html
#[derive(Debug, PartialEq, Clone)]
pub enum DataValue {
    /// Integer
    Int(SbrdInt),
    /// Real
    Real(SbrdReal),
    /// Boolean
    Bool(SbrdBool),
    /// String
    String(String),
    /// Null
    Null,
}

impl From<SbrdInt> for DataValue {
    fn from(v: SbrdInt) -> Self {
        Self::Int(v)
    }
}

impl From<SbrdReal> for DataValue {
    fn from(v: SbrdReal) -> Self {
        Self::Real(v)
    }
}

impl From<SbrdBool> for DataValue {
    fn from(v: SbrdBool) -> Self {
        Self::Bool(v)
    }
}

impl From<String> for DataValue {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}

impl From<SbrdDateTime> for DataValue {
    fn from(v: SbrdDateTime) -> Self {
        Self::String(v.format(DATE_TIME_DEFAULT_FORMAT).to_string())
    }
}

impl From<SbrdDate> for DataValue {
    fn from(v: SbrdDate) -> Self {
        Self::String(v.format(DATE_DEFAULT_FORMAT).to_string())
    }
}

impl From<SbrdTime> for DataValue {
    fn from(v: SbrdTime) -> Self {
        Self::String(v.format(TIME_DEFAULT_FORMAT).to_string())
    }
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
        deserializer.deserialize_any(Self)
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

impl std::fmt::Display for DataValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataValue::Int(v) => fmt::Display::fmt(v, f),
            DataValue::Real(v) => fmt::Display::fmt(v, f),
            DataValue::Bool(v) => fmt::Display::fmt(v, f),
            DataValue::String(v) => fmt::Display::fmt(v, f),
            DataValue::Null => write!(f, "null"),
        }
    }
}

impl FormatArgument for DataValue {
    fn supports_format(&self, specifier: &Specifier) -> bool {
        <&DataValue as FormatArgument>::supports_format(&self, specifier)
    }

    fn fmt_display(&self, f: &mut Formatter) -> fmt::Result {
        <&DataValue as FormatArgument>::fmt_display(&self, f)
    }

    fn fmt_debug(&self, f: &mut Formatter) -> fmt::Result {
        <&DataValue as FormatArgument>::fmt_debug(&self, f)
    }

    fn fmt_octal(&self, f: &mut Formatter) -> fmt::Result {
        <&DataValue as FormatArgument>::fmt_octal(&self, f)
    }

    fn fmt_lower_hex(&self, f: &mut Formatter) -> fmt::Result {
        <&DataValue as FormatArgument>::fmt_lower_hex(&self, f)
    }

    fn fmt_upper_hex(&self, f: &mut Formatter) -> fmt::Result {
        <&DataValue as FormatArgument>::fmt_upper_exp(&self, f)
    }

    fn fmt_binary(&self, f: &mut Formatter) -> fmt::Result {
        <&DataValue as FormatArgument>::fmt_binary(&self, f)
    }

    fn fmt_lower_exp(&self, f: &mut Formatter) -> fmt::Result {
        <&DataValue as FormatArgument>::fmt_lower_exp(&self, f)
    }

    fn fmt_upper_exp(&self, f: &mut Formatter) -> fmt::Result {
        <&DataValue as FormatArgument>::fmt_upper_exp(&self, f)
    }

    fn to_usize(&self) -> Result<usize, ()> {
        <&DataValue as FormatArgument>::to_usize(&self)
    }
}

impl<'a> FormatArgument for &'a DataValue {
    fn supports_format(&self, specifier: &Specifier) -> bool {
        // Not support debug format in release build.
        if !cfg!(debug_assertions) && specifier.format == Format::Debug {
            return false;
        }

        match self {
            DataValue::Int(_) | DataValue::Null => true,
            DataValue::Real(_) => matches!(
                specifier.format,
                Format::Display | Format::Debug | Format::LowerExp | Format::UpperExp
            ),
            DataValue::Bool(_) | DataValue::String(_) => {
                matches!(specifier.format, Format::Display | Format::Debug)
            }
        }
    }

    fn fmt_display(&self, f: &mut Formatter) -> fmt::Result {
        fmt::Display::fmt(*self, f)
    }

    fn fmt_debug(&self, f: &mut Formatter) -> fmt::Result {
        fmt::Debug::fmt(*self, f)
    }

    fn fmt_octal(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            DataValue::Int(v) => fmt::Octal::fmt(v, f),
            DataValue::Null => {
                // not format null value
                self.fmt_display(f)
            }
            _ => Err(fmt::Error),
        }
    }

    fn fmt_lower_hex(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            DataValue::Int(v) => fmt::LowerHex::fmt(v, f),
            DataValue::Null => {
                // not format null value
                self.fmt_display(f)
            }
            _ => Err(fmt::Error),
        }
    }

    fn fmt_upper_hex(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            DataValue::Int(v) => fmt::UpperHex::fmt(v, f),
            DataValue::Null => {
                // not format null value
                self.fmt_display(f)
            }
            _ => Err(fmt::Error),
        }
    }

    fn fmt_binary(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            DataValue::Int(v) => fmt::Binary::fmt(v, f),
            DataValue::Null => {
                // not format null value
                self.fmt_display(f)
            }
            _ => Err(fmt::Error),
        }
    }

    fn fmt_lower_exp(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            DataValue::Int(v) => fmt::LowerExp::fmt(v, f),
            DataValue::Real(v) => fmt::LowerExp::fmt(v, f),
            DataValue::Null => {
                // not format null value
                self.fmt_display(f)
            }
            _ => Err(fmt::Error),
        }
    }

    fn fmt_upper_exp(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            DataValue::Int(v) => fmt::UpperExp::fmt(v, f),
            DataValue::Real(v) => fmt::UpperExp::fmt(v, f),
            DataValue::Null => {
                // not format null value
                self.fmt_display(f)
            }
            _ => Err(fmt::Error),
        }
    }

    fn to_usize(&self) -> Result<usize, ()> {
        // Not support
     Err(())
    }
}

impl DataValue {
    /// Convert to String to use when permute with other Strings
    pub fn to_permutation_string(&self) -> String {
        match self {
            DataValue::Int(v) => v.to_string(),
            DataValue::Real(v) => v.to_string(),
            DataValue::Bool(v) => v.to_string(),
            DataValue::String(v) => v.to_string(),
            DataValue::Null => "".to_string(),
        }
    }

    /// Convert to String to use parse
    pub fn to_parse_string(&self) -> String {
        match self {
            DataValue::Int(v) => v.to_string(),
            DataValue::Real(v) => v.to_string(),
            DataValue::Bool(v) => v.to_string(),
            DataValue::String(v) => v.to_string(),
            DataValue::Null => "".to_string(),
        }
    }

    /// Format this value
    ///
    /// Support [`Rust-format syntax`]. But not support position, variable, padding with character and [`Pointer`] format (`{:p}`).
    /// [`Debug`] format is not supported in release build.
    ///
    /// # Examples
    /// ```
    /// fn main(){
    ///     use sbrd_gen::value::DataValue;
    ///
    ///     assert_eq!(Some("ignore value".to_string()), DataValue::Int(12).format("ignore value"));
    ///     assert_eq!(Some("12".to_string()), DataValue::Int(12).format("{}"));
    ///     assert_eq!(Some("{}".to_string()), DataValue::Int(12).format("{{}}"));
    ///     assert_eq!(Some("Rate= +12.35".to_string()), DataValue::Real(12.345).format("Rate={:+7.2}"));
    ///     assert_eq!(Some("Rate=+012.35".to_string()), DataValue::Real(12.345).format("Rate={:+07.2}"));
    ///     assert_eq!(Some(" aiueoあいうえお ".to_string()), DataValue::String("aiueoあいうえお".to_string()).format("{:^12}"));
    ///     assert_eq!(Some("true    ".to_string()), DataValue::Bool(true).format("{:<8}"));
    ///     assert_eq!(Some("null".to_string()), DataValue::Null.format("{:<10}"));
    /// }
    /// ```
    ///
    /// [`Rust-format syntax`]: https://doc.rust-lang.org/std/fmt/index.html#syntax
    /// [`Pointer`]: https://doc.rust-lang.org/std/fmt/trait.Pointer.html
    /// [`Debug`]: https://doc.rust-lang.org/std/fmt/trait.Debug.html
    pub fn format(&self, format: &str) -> Option<String> {
        let pos_args = [self];
        let parsed_args = ParsedFormat::parse(format, &pos_args, &NoNamedArguments);
        match parsed_args {
            Ok(args) => Some(format!("{}", args)),
            Err(_) => None,
        }
    }
}
