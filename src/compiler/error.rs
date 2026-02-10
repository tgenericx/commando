use crate::commit_message::ValidationError;

/// Compiler errors that can occur during commit message processing.
///
/// This enum represents failures across the compiler pipeline stages:
/// - lexical analysis
/// - syntax analysis
/// - semantic analysis (domain validation)
#[derive(Debug, Clone, PartialEq)]
pub enum CompileError {
    /// Error during lexical analysis (tokenization)
    LexerError(String),

    /// Error during syntax analysis (AST construction)
    ParseError(String),

    /// Error during semantic analysis (domain-level validation)
    SemanticError(ValidationError),
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::LexerError(msg) => write!(f, "Lexer error: {}", msg),
            CompileError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            CompileError::SemanticError(err) => write!(f, "Semantic error: {}", err),
        }
    }
}

impl std::error::Error for CompileError {}

impl From<ValidationError> for CompileError {
    fn from(err: ValidationError) -> Self {
        CompileError::SemanticError(err)
    }
}
