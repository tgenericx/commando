/// Input port — contract between input sources and the application.
use crate::domain::{CommitMessage, CommitType, DomainError};

/// Output of InteractiveSource — fields already parsed and individually validated.
#[derive(Debug, Clone)]
pub struct StructuredInput {
    pub commit_type: CommitType,
    pub scope: Option<String>,
    pub description: String,
    pub body: Option<String>,
    pub breaking_change: Option<String>,
    /// Refs, Closes, Co-authored-by, etc. — anything the user typed in the
    /// refs prompt. Stored as a single raw string and threaded through as a
    /// single footer entry keyed "Refs" if present.
    pub refs: Option<String>,
}

impl TryFrom<StructuredInput> for CommitMessage {
    type Error = DomainError;

    fn try_from(s: StructuredInput) -> Result<Self, DomainError> {
        let footers = match s.refs {
            Some(refs) => vec![("Refs".to_string(), refs)],
            None => vec![],
        };

        CommitMessage::new(
            s.commit_type,
            s.scope,
            s.description,
            s.body,
            s.breaking_change,
            footers,
        )
    }
}

/// Low-level collection contract — used internally by InteractiveSource.
pub trait InputSource {
    type Output;
    type Error: std::fmt::Display;
    fn collect(&self) -> Result<Self::Output, Self::Error>;
}

/// The unified trait AppController depends on.
/// Every input mode implements this — editor, direct, interactive.
pub trait CommitMessageSource {
    type Error: std::fmt::Display;
    fn resolve(&self) -> Result<CommitMessage, Self::Error>;
}
