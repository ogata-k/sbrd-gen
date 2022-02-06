use crate::generators::error::CompileError;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    info: ErrorInfo,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ErrorKind::ParseError => write!(f, "Parse error: {}", self.info),
            ErrorKind::CompileError => write!(f, "Compile error: {}", self.info),
            ErrorKind::GenerateError => write!(f, "Generate error: {}", self.info),
            ErrorKind::SerializeError => write!(f, "Serialize error: {}", self.info),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ErrorKind {
    ParseError,
    CompileError,
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
        e.into_sbrd_gen_error(ErrorKind::CompileError)
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
