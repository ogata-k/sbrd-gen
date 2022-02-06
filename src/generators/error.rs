use crate::GeneratorType;

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
