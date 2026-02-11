use std::fmt;

/// Token types for commit message compilation
///
/// Represents the atomic units of a conventional commit message.
/// Tokens are emitted by the lexer and consumed by the parser.
/// They represent structure only, not semantic correctness.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// Commit type (e.g., "feat", "fix", "docs")
    /// Contains the raw type string before validation
    Type(String),

    /// Scope identifier (e.g., "api", "parser", "auth-service")
    /// Represents the module or area affected by the commit
    Scope(String),

    /// Breaking change indicator (the "!" symbol)
    /// Signals that this commit contains breaking changes
    Breaking,

    /// Short description of the change
    /// The summary line that follows the type/scope
    Description(String),

    /// Detailed explanation of the change
    /// Multi-line text providing context and rationale
    Body(String),

    /// Footer entry with key-value structure
    /// Generic footer for metadata like "BREAKING CHANGE:", "Refs:", etc.
    /// Contains the full footer line including the key
    Footer(String),

    /// Newline token to track line boundaries
    /// Used to distinguish between single-line and multi-line sections
    Newline,

    /// End of input marker
    /// Signals that the lexer has processed all input
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Type(s) => write!(f, "Type({})", s),
            Token::Scope(s) => write!(f, "Scope({})", s),
            Token::Breaking => write!(f, "Breaking"),
            Token::Description(s) => {
                let mut preview: String = s.chars().take(30).collect();
                if s.chars().count() > 30 {
                    preview.push_str("...");
                }
                write!(f, "Description({})", preview)
            }
            Token::Body(s) => {
                let preview = if s.len() > 30 {
                    format!("{}...", &s[..30])
                } else {
                    s.clone()
                };
                write!(f, "Body({})", preview)
            }
            Token::Footer(s) => write!(f, "Footer({})", s),
            Token::Newline => write!(f, "Newline"),
            Token::Eof => write!(f, "Eof"),
        }
    }
}
