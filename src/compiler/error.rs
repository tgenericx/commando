use crate::compiler::token::Token;
use crate::validation::ValidationError;

/// Compiler errors that can occur during commit message processing.
///
/// Represents failures across the compiler pipeline:
/// - lexical analysis
/// - syntax analysis (parsing into AST)
/// - semantic analysis (domain validation)
#[derive(Debug, Clone, PartialEq)]
pub enum CompileError {
    /// Error during lexical analysis (tokenization)
    LexerError(String),

    /// Error during syntax analysis (AST construction)
    ParseError(ParseError),

    /// Error during semantic analysis (domain-level validation)
    SemanticError(ValidationError),
}

/// Errors produced specifically by the parser.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// A token was encountered that did not match what was expected
    UnexpectedToken { expected: String, found: Token },

    /// Footer line exists but is syntactically invalid
    InvalidFooter(String),
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::LexerError(msg) => write!(f, "Lexer error: {}", msg),
            CompileError::ParseError(err) => write!(f, "Parse error: {}", err),
            CompileError::SemanticError(err) => write!(f, "Semantic error: {}", err),
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken { expected, found } => {
                write!(f, "expected {}, found {}", expected, found)
            }
            ParseError::InvalidFooter(raw) => {
                write!(f, "invalid footer syntax: {}", raw)
            }
        }
    }
}

impl std::error::Error for CompileError {}
impl std::error::Error for ParseError {}

impl From<ValidationError> for CompileError {
    fn from(err: ValidationError) -> Self {
        CompileError::SemanticError(err)
    }
}
