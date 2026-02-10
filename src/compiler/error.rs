#[derive(Debug)]
pub enum CompileError {
    NotImplemented,
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Compiler not implemented")
    }
}

impl std::error::Error for CompileError {}
