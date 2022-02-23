use crate::error::{IntoSbrdError, SchemeErrorKind, SchemeResult};
use crate::SchemeBuilder;

pub trait SchemeParser {
    fn parse_from_str(input: &str) -> SchemeResult<SchemeBuilder>;
    fn parse_from_reader<R: std::io::Read>(rdr: R) -> SchemeResult<SchemeBuilder>;
}

pub struct YamlParser;
impl SchemeParser for YamlParser {
    fn parse_from_str(input: &str) -> SchemeResult<SchemeBuilder>
    where
        Self: Sized,
    {
        serde_yaml::from_str(input).map_err(|e| e.into_sbrd_gen_error(SchemeErrorKind::ParseError))
    }

    fn parse_from_reader<R: std::io::Read>(rdr: R) -> SchemeResult<SchemeBuilder> {
        serde_yaml::from_reader(rdr).map_err(|e| e.into_sbrd_gen_error(SchemeErrorKind::ParseError))
    }
}

pub struct JsonParser;
impl SchemeParser for JsonParser {
    fn parse_from_str(input: &str) -> SchemeResult<SchemeBuilder>
    where
        Self: Sized,
    {
        serde_json::from_str(input).map_err(|e| e.into_sbrd_gen_error(SchemeErrorKind::ParseError))
    }

    fn parse_from_reader<R: std::io::Read>(rdr: R) -> SchemeResult<SchemeBuilder> {
        serde_json::from_reader(rdr).map_err(|e| e.into_sbrd_gen_error(SchemeErrorKind::ParseError))
    }
}
