/// Input port — contract between input sources and the application.
use crate::domain::{CommitMessage, CommitType, DomainError};

/// Output of InteractiveSource — fields already parsed and individually validated.
/// CommitMessage::try_from(StructuredInput) just runs final domain invariant checks.
#[derive(Debug, Clone)]
pub struct StructuredInput {
    pub commit_type: CommitType,
    pub scope: Option<String>,
    pub description: String,
    pub body: Option<String>,
    pub breaking_change: Option<String>,
    #[allow(dead_code)]
    pub refs: Option<String>,
}

impl TryFrom<StructuredInput> for CommitMessage {
    type Error = DomainError;

    fn try_from(s: StructuredInput) -> Result<Self, DomainError> {
        CommitMessage::new(
            s.commit_type,
            s.scope,
            s.description,
            s.body,
            s.breaking_change,
        )
    }
}

/// Low-level collection contract — still used internally by InteractiveSource.
/// AppController no longer depends on this directly.
pub trait InputSource {
    type Output;
    type Error: std::fmt::Display;

    fn collect(&self) -> Result<Self::Output, Self::Error>;
}

/// The unified trait AppController depends on after the migration.
///
/// Every input mode — editor, direct, interactive — implements this.
/// resolve() is the single entry point: whatever the mode does internally
/// (open an editor, read a CLI arg, prompt the user), it produces a
/// CommitMessage or returns an error. AppController never sees the difference.
pub trait CommitMessageSource {
    type Error: std::fmt::Display;

    fn resolve(&self) -> Result<CommitMessage, Self::Error>;
}
