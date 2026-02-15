use std::fmt;

/// Token types for commit message compilation.
///
/// Emitted by the Lexer and consumed by the Parser.
/// Represent structure only — not semantic correctness.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// Commit type string, e.g. "feat", "fix". Not yet validated.
    Type(String),

    /// Scope string, e.g. "api", "auth-service".
    Scope(String),

    /// The '!' breaking change marker in the header.
    Breaking,

    /// The description — everything after ': ' on the header line.
    Description(String),

    /// The commit body — multi-line free text after the first blank line.
    Body(String),

    /// A raw footer line, e.g. "BREAKING CHANGE: old API removed".
    Footer(String),

    /// Line boundary marker used by the parser to track sections.
    Newline,

    /// End of input.
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Type(s) => write!(f, "Type({})", s),
            Token::Scope(s) => write!(f, "Scope({})", s),
            Token::Breaking => write!(f, "Breaking"),
            Token::Description(s) => {
                let preview: String = s.chars().take(30).collect();
                if s.chars().count() > 30 {
                    write!(f, "Description({}...)", preview)
                } else {
                    write!(f, "Description({})", preview)
                }
            }
            Token::Body(s) => {
                if s.len() > 30 {
                    write!(f, "Body({}...)", &s[..30])
                } else {
                    write!(f, "Body({})", s)
                }
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
    fn tokens_are_cloneable_and_comparable() {
        let t1 = Token::Type("feat".to_string());
        let t2 = Token::Type("feat".to_string());
        let t3 = Token::Type("fix".to_string());
        assert_eq!(t1, t2);
        assert_ne!(t1, t3);
        assert_eq!(t1, t1.clone());
    }

    #[test]
    fn display_type() {
        assert_eq!(format!("{}", Token::Type("feat".into())), "Type(feat)");
    }

    #[test]
    fn display_breaking() {
        assert_eq!(format!("{}", Token::Breaking), "Breaking");
    }

    #[test]
    fn display_truncates_long_description() {
        let token = Token::Description("a".repeat(50));
        let s = format!("{}", token);
        assert!(s.starts_with("Description("));
        assert!(s.contains("..."));
        assert!(s.len() < 60);
    }

    #[test]
    fn display_newline_and_eof() {
        assert_eq!(format!("{}", Token::Newline), "Newline");
        assert_eq!(format!("{}", Token::Eof), "Eof");
    }
}
