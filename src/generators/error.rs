use eval;

use crate::{DataValue, GeneratorType, ValueBound};

#[derive(Debug)]
pub enum CompileError {
    InvalidType(GeneratorType),
    InvalidValue(String),
    NotExistValueOf(String),
    RangeEmpty(ValueBound<DataValue>),
    EmptyChildren,
    NotExistDefaultCase,
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::InvalidType(t) => write!(f, "Invalid Type: {}", t),
            CompileError::InvalidValue(s) => write!(f, "Invalid Value: {}", s),
            CompileError::NotExistValueOf(s) => write!(f, "Not Exist Value for {}", s),
            CompileError::RangeEmpty(range) => write!(f, "Empty Range: {}", range),
            CompileError::EmptyChildren => write!(f, "Not Exist selectable children"),
            CompileError::NotExistDefaultCase => write!(f, "Not Exist default case condition"),
        }
    }
}

impl std::error::Error for CompileError {}

#[derive(Debug)]
pub enum GenerateError {
    /// eval error, replaced script, unmodified script
    FailEval(eval::Error, String, String),
    /// type name, value, unmodified script
    FailCastOfEvalScript(String, eval::Value, String),
    /// reason
    FailGenerate(String),
}

impl std::fmt::Display for GenerateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenerateError::FailEval(e, modified_script, unmodified_script) => write!(
                f,
                "Eval error: {} on evaluate \"{}\" from script \"{}\"",
                e, modified_script, unmodified_script
            ),
            GenerateError::FailCastOfEvalScript(type_name, value, script) => write!(
                f,
                "Cast Value error: `{}` as '{}' on eval script \"{}\"",
                value, type_name, script
            ),
            GenerateError::FailGenerate(s) => {
                write!(f, "Fail Generate valid data. Because {}", s)
            }
        }
    }
}

impl std::error::Error for GenerateError {}
