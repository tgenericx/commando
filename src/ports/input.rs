/// Input port — contract between input sources and the application.
///
/// InteractiveSource produces StructuredInput (fields already parsed).
/// Future DirectSource will have its own Output type (raw string).
/// AppController is generic over InputSource, never tied to one source.
use crate::domain::{CommitType, DomainError};

/// Output of InteractiveSource.
///
/// All fields are already parsed and individually validated at prompt time.
/// No further lexing or parsing is needed — CommitMessage::try_from() just
/// runs the final domain invariant checks.
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

/// The source-agnostic collection contract.
///
/// Each source defines its own Output and Error types.
/// AppController constrains Output = StructuredInput for the interactive path.
pub trait InputSource {
    type Output;
    type Error: std::fmt::Display;

    fn collect(&self) -> Result<Self::Output, Self::Error>;
}

/// Convenience — lets AppController call CommitMessage::try_from(input)
/// directly after collection.
impl TryFrom<StructuredInput> for crate::domain::CommitMessage {
    type Error = DomainError;

    fn try_from(s: StructuredInput) -> Result<Self, DomainError> {
        crate::domain::CommitMessage::new(
            s.commit_type,
            s.scope,
            s.description,
            s.body,
            s.breaking_change,
            // s.refs: not yet modelled in CommitMessage.
            // Add a refs field there when the footer renderer is ready.
        )
    }
}
