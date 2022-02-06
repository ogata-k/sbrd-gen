use crate::{GeneratorType, ValueBound};

#[derive(Debug)]
pub enum CompileError {
    InvalidType(GeneratorType),
    InvalidValue(String),
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::InvalidType(t) => write!(f, "Invalid Type: {}", t),
            CompileError::InvalidValue(s) => write!(f, "Invalid Value: {}", s),
        }
    }
}

impl std::error::Error for CompileError {}

#[derive(Debug)]
pub enum GenerateError {
    // 文字列に変換して範囲を持つようにする
    RangeEmpty(ValueBound<String>),
}

impl std::fmt::Display for GenerateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenerateError::RangeEmpty(range) => write!(f, "Empty Range: {}", range),
        }
    }
}

impl std::error::Error for GenerateError {}
