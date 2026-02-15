use crate::compiler::token::Token;

/// Errors produced by the compiler pipeline.
///
/// Lexer errors: malformed input structure (missing ':', unclosed parens, empty header).
/// Parse errors: token stream doesn't match the grammar.
///
/// Neither error type carries DomainError — semantic validation
/// (valid type string, description length, scope charset) is the domain's job.
#[derive(Debug, Clone, PartialEq)]
pub enum CompileError {
    /// Error during lexical analysis.
    Lex(String),

    /// Error during parsing (token stream doesn't match grammar).
    Parse(ParseError),
}

/// Specific parse failures.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// A token was encountered that did not match what was expected.
    UnexpectedToken { expected: String, found: Token },

    /// Footer line exists but is syntactically invalid.
    InvalidFooter(String),
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::Lex(msg) => write!(f, "Lexer error: {}", msg),
            CompileError::Parse(err) => write!(f, "Parse error: {}", err),
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
                write!(f, "invalid footer syntax: '{}'", raw)
            }
        }
    }
}

impl std::error::Error for CompileError {}
impl std::error::Error for ParseError {}
