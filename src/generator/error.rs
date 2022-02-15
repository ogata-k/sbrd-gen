use crate::builder::ValueBound;
use crate::eval::EvalError;
use crate::value::{DataValue, DataValueMap};
use crate::GeneratorType;

#[derive(Debug)]
pub enum CompileError {
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

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::InvalidType(t) => write!(f, "Invalid Type: {}", t),
            CompileError::InvalidValue(s) => write!(f, "Invalid Value: {}", s),
            CompileError::NotExistValueOf(s) => write!(f, "Not Exist Value for {}", s),
            CompileError::FailParseValue(s, t, e) => {
                write!(f, "Fail Parse {} as {} with error: {}", s, t, e)
            }
            CompileError::RangeEmpty(range) => write!(f, "Empty Range: {}", range),
            CompileError::EmptyChildren => write!(f, "Not Exist selectable children"),
            CompileError::EmptySelectValues => write!(f, "Not Exist selectable values"),
            CompileError::EmptyRandomize => {
                write!(f, "Not Exist selectable children xor (chars, values, file)")
            }
            CompileError::NotExistDefaultCase => write!(f, "Not Exist default case condition"),
            CompileError::AllWeightsZero => write!(f, "All weights are zero"),
            CompileError::FileError(fe) => write!(f, "File Error: {}", fe),
            CompileError::FailBuildDistribution(dn, e) => {
                write!(f, "Fail build {} distribution with error: {}", dn, e)
            }
        }
    }
}

impl std::error::Error for CompileError {}

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
