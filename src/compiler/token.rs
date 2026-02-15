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

impl Token {
    /// Returns a human-readable description of the token type
    pub fn type_name(&self) -> &'static str {
        match self {
            Token::Type(_) => "Type",
            Token::Scope(_) => "Scope",
            Token::Breaking => "Breaking",
            Token::Description(_) => "Description",
            Token::Body(_) => "Body",
            Token::Footer(_) => "Footer",
            Token::Newline => "Newline",
            Token::Eof => "Eof",
        }
    }

    /// Extracts the string value from tokens that contain data
    pub fn value(&self) -> Option<&str> {
        match self {
            Token::Type(s) => Some(s),
            Token::Scope(s) => Some(s),
            Token::Description(s) => Some(s),
            Token::Body(s) => Some(s),
            Token::Footer(s) => Some(s),
            Token::Breaking | Token::Newline | Token::Eof => None,
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_enum_can_be_instantiated() {
        let _type_token = Token::Type("feat".to_string());
        let _scope_token = Token::Scope("api".to_string());
        let _breaking_token = Token::Breaking;
        let _description_token = Token::Description("add feature".to_string());
        let _body_token = Token::Body("detailed explanation".to_string());
        let _footer_token = Token::Footer("BREAKING CHANGE: details".to_string());
        let _newline_token = Token::Newline;
        let _eof_token = Token::Eof;
    }

    #[test]
    fn token_type_name_returns_correct_name() {
        assert_eq!(Token::Type("feat".to_string()).type_name(), "Type");
        assert_eq!(Token::Scope("api".to_string()).type_name(), "Scope");
        assert_eq!(Token::Breaking.type_name(), "Breaking");
        assert_eq!(
            Token::Description("desc".to_string()).type_name(),
            "Description"
        );
        assert_eq!(Token::Body("body".to_string()).type_name(), "Body");
        assert_eq!(Token::Footer("footer".to_string()).type_name(), "Footer");
        assert_eq!(Token::Newline.type_name(), "Newline");
        assert_eq!(Token::Eof.type_name(), "Eof");
    }

    #[test]
    fn token_value_extracts_string_content() {
        assert_eq!(Token::Type("feat".to_string()).value(), Some("feat"));
        assert_eq!(Token::Scope("api".to_string()).value(), Some("api"));
        assert_eq!(Token::Breaking.value(), None);
        assert_eq!(Token::Newline.value(), None);
        assert_eq!(Token::Eof.value(), None);
    }

    #[test]
    fn token_display_format_works() {
        let type_token = Token::Type("feat".to_string());
        assert_eq!(format!("{}", type_token), "Type(feat)");

        let breaking_token = Token::Breaking;
        assert_eq!(format!("{}", breaking_token), "Breaking");

        let newline_token = Token::Newline;
        assert_eq!(format!("{}", newline_token), "Newline");
    }

    #[test]
    fn token_display_truncates_long_descriptions() {
        let long_desc = "a".repeat(50);
        let token = Token::Description(long_desc);
        let display = format!("{}", token);

        assert!(display.starts_with("Description("));
        assert!(display.contains("..."));
        assert!(display.len() < 60); // Should be truncated
    }

    #[test]
    fn tokens_are_cloneable() {
        let token = Token::Type("feat".to_string());
        let cloned = token.clone();
        assert_eq!(token, cloned);
    }

    #[test]
    fn tokens_are_comparable() {
        let token1 = Token::Type("feat".to_string());
        let token2 = Token::Type("feat".to_string());
        let token3 = Token::Type("fix".to_string());

        assert_eq!(token1, token2);
        assert_ne!(token1, token3);
    }
}
