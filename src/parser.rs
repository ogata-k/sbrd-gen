#![deny(missing_debug_implementations)]

use crate::error::{IntoSbrdError, SchemaErrorKind, SchemaResult};
use crate::SchemaBuilder;

pub trait SchemaParser {
    fn parse_from_str(input: &str) -> SchemaResult<SchemaBuilder>;
    fn parse_from_reader<R: std::io::Read>(rdr: R) -> SchemaResult<SchemaBuilder>;
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub struct YamlParser;
impl SchemaParser for YamlParser {
    fn parse_from_str(input: &str) -> SchemaResult<SchemaBuilder>
    where
        Self: Sized,
    {
        serde_yaml::from_str(input).map_err(|e| e.into_sbrd_gen_error(SchemaErrorKind::ParseError))
    }

    fn parse_from_reader<R: std::io::Read>(rdr: R) -> SchemaResult<SchemaBuilder> {
        serde_yaml::from_reader(rdr).map_err(|e| e.into_sbrd_gen_error(SchemaErrorKind::ParseError))
    }
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub struct JsonParser;
impl SchemaParser for JsonParser {
    fn parse_from_str(input: &str) -> SchemaResult<SchemaBuilder>
    where
        Self: Sized,
    {
        serde_json::from_str(input).map_err(|e| e.into_sbrd_gen_error(SchemaErrorKind::ParseError))
    }

    fn parse_from_reader<R: std::io::Read>(rdr: R) -> SchemaResult<SchemaBuilder> {
        serde_json::from_reader(rdr).map_err(|e| e.into_sbrd_gen_error(SchemaErrorKind::ParseError))
    }
}
