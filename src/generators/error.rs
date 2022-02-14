use crate::eval::EvalError;
use crate::{DataValue, DataValueMap, GeneratorType, ValueBound};

#[derive(Debug)]
pub enum CompileError {
    InvalidType(GeneratorType),
    InvalidValue(String),
    /// parse string, type, error string
    FailParseValue(String, String, String),
    NotExistValueOf(String),
    RangeEmpty(ValueBound<DataValue>),
    EmptyChildren,
    EmptySelectValues,
    EmptyRandomize,
    NotExistDefaultCase,
    AllWeightsZero,
    FileError(std::io::Error),
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::InvalidType(t) => write!(f, "Invalid Type: {}", t),
            CompileError::InvalidValue(s) => write!(f, "Invalid Value: {}", s),
            CompileError::FailParseValue(s, t, e) => {
                write!(f, "Fail Parse {} as {} with error: {}", s, t, e)
            }
            CompileError::NotExistValueOf(s) => write!(f, "Not Exist Value for {}", s),
            CompileError::RangeEmpty(range) => write!(f, "Empty Range: {}", range),
            CompileError::EmptyChildren => write!(f, "Not Exist selectable children"),
            CompileError::EmptySelectValues => write!(f, "Not Exist selectable values"),
            CompileError::EmptyRandomize => {
                write!(f, "Not Exist selectable children xor (chars, values, file)")
            }
            CompileError::NotExistDefaultCase => write!(f, "Not Exist default case condition"),
            CompileError::AllWeightsZero => write!(f, "All weights are zero"),
            CompileError::FileError(fe) => write!(f, "File Error: {}", fe),
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
