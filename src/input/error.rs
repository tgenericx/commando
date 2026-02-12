use crate::compiler::error::CompileError;

#[derive(Debug)]
pub enum InputError {
    Compilation(CompileError),
    Io(std::io::Error),
    Cancelled,
    Empty,
}

impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputError::Compilation(e) => write!(f, "{}", e),
            InputError::Io(e) => write!(f, "File error: {}", e),
            InputError::Cancelled => write!(f, "Cancelled by user"),
            InputError::Empty => write!(f, "Empty commit message"),
        }
    }
}

impl From<CompileError> for InputError {
    fn from(err: CompileError) -> Self {
        InputError::Compilation(err)
    }
}

impl From<std::io::Error> for InputError {
    fn from(err: std::io::Error) -> Self {
        InputError::Io(err)
    }
}
