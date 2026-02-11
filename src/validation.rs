use crate::commit_types::CommitType;

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    InvalidCommitType(String),
    EmptyDescription,
    DescriptionTooLong(usize),
    InvalidScope(String),
    EmptyBreakingChange,
    BreakingChangeMismatch,
    InvalidFooter(String),
    DuplicateFooter(String),
    InvalidIssueReference(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InvalidCommitType(t) => write!(
                f,
                "Invalid commit type: '{}'. Must be one of: {}",
                t,
                CommitType::all_as_str().join(", ")
            ),
            ValidationError::EmptyDescription => write!(f, "Description cannot be empty"),
            ValidationError::DescriptionTooLong(len) => write!(
                f,
                "Description is too long ({} characters). Maximum is 72 characters",
                len
            ),
            ValidationError::InvalidScope(s) => write!(
                f,
                "Invalid scope: '{}'. Scope must be alphanumeric with hyphens/underscores",
                s
            ),
            ValidationError::EmptyBreakingChange => {
                write!(f, "Breaking change description cannot be empty")
            }
            ValidationError::BreakingChangeMismatch => write!(
                f,
                "Breaking change mismatch: header '!' and BREAKING CHANGE footer must both be present"
            ),
            ValidationError::InvalidFooter(line) => write!(
                f,
                "Invalid footer line: '{}'. Expected format 'KEY: value'",
                line
            ),
            ValidationError::DuplicateFooter(key) => write!(
                f,
                "Duplicate footer key detected: '{}'. Footer keys must be unique",
                key
            ),
            ValidationError::InvalidIssueReference(value) => write!(
                f,
                "Invalid issue reference: '{}'. Expected format like '#123'",
                value
            ),
        }
    }
}

impl std::error::Error for ValidationError {}
