use crate::builder::ValueBound;
use crate::eval::EvalError;
use crate::value::{DataValue, DataValueMap};
use crate::GeneratorType;
use std::borrow::Borrow;

#[derive(Debug)]
pub struct SbrdError {
    kind: SbrdErrorKind,
    info: SbrdErrorInfo,
}

impl std::fmt::Display for SbrdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            SbrdErrorKind::ParseError => write!(f, "Parse error: {}", self.info),
            SbrdErrorKind::BuildError => write!(f, "Build error: {}", self.info),
            SbrdErrorKind::GenerateError => write!(f, "Generate error: {}", self.info),
            SbrdErrorKind::SerializeError => write!(f, "Serialize error: {}", self.info),
        }
    }
}

impl std::error::Error for SbrdError {}

impl SbrdError {
    pub fn is_kind_of(&self, kind: SbrdErrorKind) -> bool {
        self.kind == kind
    }

    pub fn get_kind(&self) -> SbrdErrorKind {
        self.kind
    }

    pub fn get_error_info(&self) -> &dyn std::error::Error {
        self.info.0.borrow()
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SbrdErrorKind {
    ParseError,
    BuildError,
    GenerateError,
    SerializeError,
}

#[derive(Debug)]
struct SbrdErrorInfo(Box<dyn std::error::Error>);

impl std::fmt::Display for SbrdErrorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<BuildError> for SbrdError {
    fn from(e: BuildError) -> Self {
        e.into_sbrd_gen_error(SbrdErrorKind::BuildError)
    }
}

pub trait IntoSbrdError: 'static + std::error::Error + Sized {
    fn into_sbrd_gen_error(self, kind: SbrdErrorKind) -> SbrdError {
        SbrdError {
            kind,
            info: SbrdErrorInfo(Box::new(self)),
        }
    }
}

impl<E> IntoSbrdError for E where E: 'static + std::error::Error + Sized {}

pub type SbrdGenResult<T> = std::result::Result<T, SbrdError>;

#[derive(Debug)]
pub enum BuildError {
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
    FileError(std::io::Error),
    /// Distribution name, error
    FailBuildDistribution(String, String),
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
            BuildError::FileError(fe) => write!(f, "File Error: {}", fe),
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
    FailEval(EvalError, String, DataValueMap),
    /// reason
    FailGenerate(String),
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
        }
    }
}

impl std::error::Error for GenerateError {}
