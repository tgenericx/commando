/// Domain Error Types
///
/// Defines all possible validation errors that can occur in the domain layer.
use crate::domain::commit_type::CommitType;

#[derive(Debug, Clone, PartialEq)]
pub enum DomainError {
    // Validation errors
    InvalidCommitType(String),
    EmptyDescription,
    DescriptionTooLong(usize),
    InvalidScope(String),
    EmptyBreakingChange,
    EmptyBody,
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Validation errors
            DomainError::InvalidCommitType(t) => {
                let valid_types = CommitType::all_as_str().join(", ");
                write!(
                    f,
                    "Invalid commit type: '{}'. Must be one of: {}",
                    t, valid_types
                )
            }
            DomainError::EmptyDescription => {
                write!(f, "Description cannot be empty")
            }
            DomainError::DescriptionTooLong(len) => {
                write!(
                    f,
                    "Description is too long ({} characters). Maximum is 72 characters",
                    len
                )
            }
            DomainError::InvalidScope(s) => {
                write!(
                    f,
                    "Invalid scope: '{}'. Scope must be alphanumeric with hyphens/underscores",
                    s
                )
            }
            DomainError::EmptyBreakingChange => {
                write!(f, "Breaking change description cannot be empty")
            }
            DomainError::EmptyBody => {
                write!(f, "Body cannot be empty if provided")
            }
        }
    }
}

impl std::error::Error for DomainError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_error_display_invalid_commit_type() {
        let error = DomainError::InvalidCommitType("invalid".to_string());
        let expected = "Invalid commit type: 'invalid'. Must be one of: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert";
        assert_eq!(error.to_string(), expected);
    }

    #[test]
    fn domain_error_display_empty_description() {
        let error = DomainError::EmptyDescription;
        assert_eq!(error.to_string(), "Description cannot be empty");
    }

    #[test]
    fn domain_error_display_description_too_long() {
        let error = DomainError::DescriptionTooLong(100);
        assert_eq!(
            error.to_string(),
            "Description is too long (100 characters). Maximum is 72 characters"
        );
    }

    #[test]
    fn domain_error_display_invalid_scope() {
        let error = DomainError::InvalidScope("invalid!".to_string());
        assert_eq!(
            error.to_string(),
            "Invalid scope: 'invalid!'. Scope must be alphanumeric with hyphens/underscores"
        );
    }

    #[test]
    fn domain_error_display_empty_breaking_change() {
        let error = DomainError::EmptyBreakingChange;
        assert_eq!(
            error.to_string(),
            "Breaking change description cannot be empty"
        );
    }

    #[test]
    fn domain_error_display_empty_body() {
        let error = DomainError::EmptyBody;
        assert_eq!(error.to_string(), "Body cannot be empty if provided");
    }
}
