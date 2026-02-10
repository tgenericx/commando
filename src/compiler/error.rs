use crate::commit_message::ValidationError;

/// Compiler errors that can occur during commit message processing
#[derive(Debug, Clone, PartialEq)]
pub enum CompileError {
    /// Error during lexical analysis (tokenization)
    LexerError(String),

    /// Error during parsing (AST construction)
    ParseError(String),

    /// Error during semantic validation
    SemanticError(String),

    /// Error during code generation
    CodeGenError(String),

    NotImplemented,
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::LexerError(msg) => write!(f, "Lexer error: {}", msg),
            CompileError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            CompileError::SemanticError(msg) => write!(f, "Semantic error: {}", msg),
            CompileError::CodeGenError(msg) => write!(f, "Code generation error: {}", msg),
            CompileError::NotImplemented => write!(f, "Feature not yet implemented"),
        }
    }
}

impl std::error::Error for CompileError {}

impl From<ValidationError> for CompileError {
    fn from(err: ValidationError) -> Self {
        CompileError::SemanticError(err.to_string())
    }
}
