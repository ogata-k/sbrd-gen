use eval;

use crate::{GeneratorType, ValueBound};

#[derive(Debug)]
pub enum CompileError {
    InvalidType(GeneratorType),
    InvalidValue(String),
    NotExistValue(String),
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::InvalidType(t) => write!(f, "Invalid Type: {}", t),
            CompileError::InvalidValue(s) => write!(f, "Invalid Value: {}", s),
            CompileError::NotExistValue(s) => write!(f, "Not Exist Value: {}", s),
        }
    }
}

impl std::error::Error for CompileError {}

#[derive(Debug)]
pub enum GenerateError {
    // 文字列に変換して範囲を持つようにする
    RangeEmpty(ValueBound<String>),
    EvalError(eval::Error, String, String),
    FailGenerate(String),
}

impl std::fmt::Display for GenerateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenerateError::RangeEmpty(range) => write!(f, "Empty Range: {}", range),
            GenerateError::EvalError(e, modified_script, unmodified_script) => write!(
                f,
                "Eval error: {} on evaluate \"{}\" from script\"{}\"",
                e, modified_script, unmodified_script
            ),
            GenerateError::FailGenerate(s) => {
                write!(f, "Fail Generate valid data. But generate {}", s)
            }
        }
    }
}

impl std::error::Error for GenerateError {}
