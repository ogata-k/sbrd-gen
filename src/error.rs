use crate::generator::CompileError;
use std::borrow::Borrow;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    info: ErrorInfo,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ErrorKind::ParseError => write!(f, "Parse error: {}", self.info),
            ErrorKind::BuildError => write!(f, "Build error: {}", self.info),
            ErrorKind::GenerateError => write!(f, "Generate error: {}", self.info),
            ErrorKind::SerializeError => write!(f, "Serialize error: {}", self.info),
        }
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn is_kind_of(&self, kind: ErrorKind) -> bool {
        self.kind == kind
    }

    pub fn get_kind(&self) -> ErrorKind {
        self.kind
    }

    pub fn get_error_info(&self) -> &dyn std::error::Error {
        self.info.0.borrow()
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ErrorKind {
    ParseError,
    BuildError,
    GenerateError,
    SerializeError,
}

#[derive(Debug)]
struct ErrorInfo(Box<dyn std::error::Error>);

impl std::fmt::Display for ErrorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<CompileError> for Error {
    fn from(e: CompileError) -> Self {
        e.into_sbrd_gen_error(ErrorKind::BuildError)
    }
}

pub trait SbrdGenError: 'static + std::error::Error + Sized {
    fn into_sbrd_gen_error(self, kind: ErrorKind) -> Error {
        Error {
            kind,
            info: ErrorInfo(Box::new(self)),
        }
    }
}

impl<E> SbrdGenError for E where E: 'static + std::error::Error + Sized {}

pub type SbrdGenResult<T> = std::result::Result<T, Error>;
