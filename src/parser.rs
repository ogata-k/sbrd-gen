#![deny(missing_debug_implementations)]
//! Module for parser

use crate::error::{IntoSbrdError, SchemaErrorKind, SchemaResult};
use crate::SchemaBuilder;

/// Trait of parser for a schema
pub trait SchemaParser {
    /// Parse from [`&str`]
    ///
    /// [`&str`]: https://doc.rust-lang.org/stable/std/str/
    fn parse_from_str(input: &str) -> SchemaResult<SchemaBuilder>;

    /// Parse from [`Read`]
    ///
    /// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
    fn parse_from_reader<R: std::io::Read>(rdr: R) -> SchemaResult<SchemaBuilder>;
}

/// Parser for a schema written as Yaml
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

/// Parser for a schema written as Json
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
