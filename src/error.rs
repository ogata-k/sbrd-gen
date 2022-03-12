#![deny(missing_debug_implementations)]
//! Module for errors used in this crate

use crate::builder::ValueBound;
use crate::eval::EvalError;
use crate::value::{DataValue, DataValueMap};
use crate::GeneratorType;
use std::borrow::Borrow;

/// A Error for a Schema
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
    /// Check error's kind
    pub fn is_kind_of(&self, kind: SchemaErrorKind) -> bool {
        self.kind == kind
    }

    /// Get error's kind
    pub fn get_kind(&self) -> SchemaErrorKind {
        self.kind
    }

    /// Get error's information
    pub fn get_error_info(&self) -> &dyn std::error::Error {
        self.info.0.borrow()
    }
}

/// Kinds of error for a Schema
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SchemaErrorKind {
    /// Error kind for parser
    ParseError,
    /// Error kind for generator builder
    BuildError,
    /// Error kind for generating a value
    GenerateError,
    /// Error kind for writing to output
    OutputError,
}

/// Error information
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

/// Trait for convert to [`SchemaError`] from other error
///
/// [`SchemaError`]: ./struct.SchemaError.html
pub trait IntoSbrdError: 'static + std::error::Error + Sized {
    /// Converter function to [`SchemaError`] from other error
    ///
    /// [`SchemaError`]: ./struct.SchemaError.html
    fn into_sbrd_gen_error(self, kind: SchemaErrorKind) -> SchemaError {
        SchemaError {
            kind,
            info: SchemaErrorInfo(Box::new(self)),
        }
    }
}

impl<E> IntoSbrdError for E where E: 'static + std::error::Error + Sized {}

/// Alias of [`Result`] type
///
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
pub type SchemaResult<T> = std::result::Result<T, SchemaError>;

/// Error for generator builder
#[derive(Debug)]
pub enum BuildError {
    /// Specified `keys` in the Schema is not unique.
    ///
    /// # Arguments
    /// * 0: Specified `keys`
    SpecifiedKeyNotUnique(Vec<String>),

    /// Specified `key` does not exist in generated values
    ///
    /// # Arguments
    /// * 0: Specified `key`
    /// * 1: Keys of generated values
    NotExistSpecifiedKey(String, Vec<String>),

    /// Specified `key` in the Schema already exist.
    ///
    /// # Arguments
    /// * 0: Specified `key`
    AlreadyExistKey(String),

    /// Specified `type` in thea Schema is not valid type.
    ///
    /// # Arguments
    /// * 0: Specified `type`
    InvalidType(GeneratorType),

    /// Specified value in the Schema is not valid value.
    ///
    /// # Arguments
    /// * 0: Specified value's string
    InvalidValue(String),

    /// Specified value's key does not exist in the Schema.
    ///
    /// # Arguments
    /// * 0: Specified value's key
    NotExistValueOf(String),

    /// Fail parse specified value in the Schema.
    ///
    /// # Arguments
    /// * 0: Parse target value
    /// * 1: Parse target type
    /// * 2: Parse error message
    FailParseValue(String, String, String),

    /// Specified `range` in the Schema is empty range.
    ///
    /// # Arguments
    /// * 0: Range bound
    RangeEmpty(ValueBound<DataValue>),

    /// Available option is one in the list. But none or many are specified.
    ///
    /// # Arguments
    /// * 0: Available options
    OnlyOneOptionSpecifiedNot(Vec<String>),

    /// Specified `children` in the Schema does not exist or does not have child.
    EmptySelectableChildren,

    /// Specified `values` in the Schema does not exist or does not have value.
    EmptySelectValues,

    /// Specified randomize values at the keys: `children`, `chars`, `values`, `filepath` is empty.
    EmptySelectable,

    /// Specified default case at the key `case` in the Schema at a child generator does not exist
    NotExistDefaultCase,

    /// Specified some of values at the key `weight` in the Schema at a child generator does not exist
    AllWeightsZero,

    /// Input/Output Error for a file.
    ///
    /// # Arguments
    /// * 0: Error information
    /// * 1: Occurred filepath
    FileError(std::io::Error, std::path::PathBuf),

    /// Error for fail build distribution with the specified parameters at the key `parameters` in the Schema
    ///
    /// # Arguments
    /// * 0: Name of the distribution
    /// * 1: Error information
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
            BuildError::OnlyOneOptionSpecifiedNot(options) => {
                write!(f, "Available option is only one in {:?}", options)
            }
            BuildError::EmptySelectableChildren => write!(f, "Selectable children is empty"),
            BuildError::EmptySelectValues => write!(f, "Selectable values is empty"),
            BuildError::EmptySelectable => {
                write!(f, "Selectable children or values is empty")
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

/// Error for generator builder
#[derive(Debug)]
pub enum GenerateError {
    /// Evaluate error while generating value
    ///
    /// # Arguments
    /// * 0: Error information
    /// * 1: Script
    /// * 2: Context key-values
    FailEval(EvalError, String, DataValueMap<String>),

    /// Error for generate value while generating value
    ///
    /// # Arguments
    /// * 0: Error message
    FailGenerate(String),

    /// Not found generate values at the key while generating value
    ///
    /// # Arguments
    /// * 0: Key
    /// * 1: Generated values
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
