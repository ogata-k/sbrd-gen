#![deny(missing_debug_implementations)]

use crate::builder::ValueBound;
use crate::eval::EvalError;
use crate::value::{DataValue, DataValueMap};
use crate::GeneratorType;
use std::borrow::Borrow;

#[derive(Debug)]
pub struct SchemaError {
    kind: SchemaErrorKind,
    info: SchemaErrorInfo,
}

impl std::fmt::Display for SchemaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            SchemaErrorKind::ParseError => write!(f, "Parse error: {}", self.info),
            SchemaErrorKind::BuildError => write!(f, "Build error: {}", self.info),
            SchemaErrorKind::GenerateError => write!(f, "Generate error: {}", self.info),
            SchemaErrorKind::OutputError => write!(f, "Output error: {}", self.info),
        }
    }
}

impl std::error::Error for SchemaError {}

impl SchemaError {
    pub fn is_kind_of(&self, kind: SchemaErrorKind) -> bool {
        self.kind == kind
    }

    pub fn get_kind(&self) -> SchemaErrorKind {
        self.kind
    }

    pub fn get_error_info(&self) -> &dyn std::error::Error {
        self.info.0.borrow()
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SchemaErrorKind {
    ParseError,
    BuildError,
    GenerateError,
    OutputError,
}

#[derive(Debug)]
struct SchemaErrorInfo(Box<dyn std::error::Error>);

impl std::fmt::Display for SchemaErrorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<BuildError> for SchemaError {
    fn from(e: BuildError) -> Self {
        e.into_sbrd_gen_error(SchemaErrorKind::BuildError)
    }
}

pub trait IntoSbrdError: 'static + std::error::Error + Sized {
    fn into_sbrd_gen_error(self, kind: SchemaErrorKind) -> SchemaError {
        SchemaError {
            kind,
            info: SchemaErrorInfo(Box::new(self)),
        }
    }
}

impl<E> IntoSbrdError for E where E: 'static + std::error::Error + Sized {}

pub type SchemaResult<T> = std::result::Result<T, SchemaError>;

#[derive(Debug)]
pub enum BuildError {
    SpecifiedKeyNotUnique(Vec<String>),
    NotExistSpecifiedKey(String, Vec<String>),
    AlreadyExistKey(String),
    InvalidType(GeneratorType),
    InvalidValue(String),
    NotExistValueOf(String),
    /// parse string, type, error string
    FailParseValue(String, String, String),
    RangeEmpty(ValueBound<DataValue>),
    EmptyChildren,
    EmptySelectValues,
    EmptyRandomize,
    NotExistDefaultCase,
    AllWeightsZero,
    FileError(std::io::Error, std::path::PathBuf),
    /// Distribution name, error
    FailBuildDistribution(String, String),
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::SpecifiedKeyNotUnique(keys) => {
                write!(f, "Not Unique Specified Keys: {:?}", keys)
            }
            BuildError::NotExistSpecifiedKey(key, keys) => {
                write!(f, "Not Exist Key \"{}\" in {:?}", key, keys)
            }
            BuildError::AlreadyExistKey(k) => write!(f, "Already Exist Key: {}", k),
            BuildError::InvalidType(t) => write!(f, "Invalid Type: {}", t),
            BuildError::InvalidValue(s) => write!(f, "Invalid Value: {}", s),
            BuildError::NotExistValueOf(s) => write!(f, "Not Exist Value for {}", s),
            BuildError::FailParseValue(s, t, e) => {
                write!(f, "Fail Parse {} as {} with error: {}", s, t, e)
            }
            BuildError::RangeEmpty(range) => write!(f, "Empty Range: {}", range),
            BuildError::EmptyChildren => write!(f, "Not Exist selectable children"),
            BuildError::EmptySelectValues => write!(f, "Not Exist selectable values"),
            BuildError::EmptyRandomize => {
                write!(f, "Not Exist selectable children xor (chars, values, file)")
            }
            BuildError::NotExistDefaultCase => write!(f, "Not Exist default case condition"),
            BuildError::AllWeightsZero => write!(f, "All weights are zero"),
            BuildError::FileError(fe, path) => write!(
                f,
                "File Error: {} at filepath `{}`",
                fe,
                path.as_path().display()
            ),
            BuildError::FailBuildDistribution(dn, e) => {
                write!(f, "Fail build {} distribution with error: {}", dn, e)
            }
        }
    }
}

impl std::error::Error for BuildError {}

#[derive(Debug)]
pub enum GenerateError {
    /// eval error, unmodified script, context
    FailEval(EvalError, String, DataValueMap<String>),
    /// reason
    FailGenerate(String),
    /// Key name, Generated values
    NotExistGeneratedKey(String, DataValueMap<String>),
}

impl std::fmt::Display for GenerateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenerateError::FailEval(e, script, context) => {
                write!(
                    f,
                    "Fail Evaluate Script \"{}\" in context {:?} with error: {}",
                    script, context, e
                )
            }
            GenerateError::FailGenerate(s) => {
                write!(f, "Fail Generate valid data. Because {}", s)
            }
            GenerateError::NotExistGeneratedKey(key, values) => {
                write!(f, "Not exist key \"{}\" in {:?}", key, values)
            }
        }
    }
}

impl std::error::Error for GenerateError {}
