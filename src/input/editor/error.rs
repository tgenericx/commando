use crate::compiler::CompileError;
use crate::domain::DomainError;

#[derive(Debug)]
pub enum EditorError {
    /// Could not create or write the temp file.
    TempFile(String),

    /// Could not resolve or spawn the editor process.
    SpawnFailed { editor: String, reason: String },

    /// Editor process exited with a non-zero status code.
    EditorFailed(String),

    /// Could not read the file after the editor closed.
    ReadFailed(String),

    /// User explicitly aborted (chose not to edit again).
    Aborted,

    /// The content compiled but failed domain validation.
    Domain(DomainError),

    /// The content failed to compile (structural / syntax error).
    Compile(CompileError),
}

impl std::fmt::Display for EditorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditorError::TempFile(e) => write!(f, "Failed to create temp file: {}", e),
            EditorError::SpawnFailed { editor, reason } => {
                write!(f, "Failed to launch '{}': {}", editor, reason)
            }
            EditorError::EditorFailed(editor) => {
                write!(f, "Editor '{}' exited with an error", editor)
            }
            EditorError::ReadFailed(e) => write!(f, "Failed to read temp file: {}", e),
            EditorError::Aborted => write!(f, "Commit aborted"),
            EditorError::Domain(e) => write!(f, "{}", e),
            EditorError::Compile(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for EditorError {}

impl From<DomainError> for EditorError {
    fn from(e: DomainError) -> Self {
        EditorError::Domain(e)
    }
}

impl From<CompileError> for EditorError {
    fn from(e: CompileError) -> Self {
        EditorError::Compile(e)
    }
}
