use crate::compiler::CompileError;
use crate::domain::DomainError;

#[derive(Debug)]
pub enum DirectError {
    /// The message string failed to compile (structural / syntax error).
    Compile(CompileError),

    /// The message compiled but failed domain validation.
    Domain(DomainError),
}

impl std::fmt::Display for DirectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectError::Compile(e) => write!(f, "{}", e),
            DirectError::Domain(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for DirectError {}

impl From<CompileError> for DirectError {
    fn from(e: CompileError) -> Self {
        DirectError::Compile(e)
    }
}

impl From<DomainError> for DirectError {
    fn from(e: DomainError) -> Self {
        DirectError::Domain(e)
    }
}
