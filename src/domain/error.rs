/// Domain Error Types
///
/// Defines all possible validation errors that can occur in the domain layer.
use crate::domain::commit_type::CommitType;

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    InvalidCommitType(String),
    EmptyDescription,
    DescriptionTooLong(usize),
    InvalidScope(String),
    EmptyBreakingChange,
    EmptyBody,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InvalidCommitType(t) => {
                let valid_types = CommitType::all_as_str().join(", ");
                write!(
                    f,
                    "Invalid commit type: '{}'. Must be one of: {}",
                    t, valid_types
                )
            }
            ValidationError::EmptyDescription => {
                write!(f, "Description cannot be empty")
            }
            ValidationError::DescriptionTooLong(len) => {
                write!(
                    f,
                    "Description is too long ({} characters). Maximum is 72 characters",
                    len
                )
            }
            ValidationError::InvalidScope(s) => {
                write!(
                    f,
                    "Invalid scope: '{}'. Scope must be alphanumeric with hyphens/underscores",
                    s
                )
            }
            ValidationError::EmptyBreakingChange => {
                write!(f, "Breaking change description cannot be empty")
            }
            ValidationError::EmptyBody => {
                write!(f, "Body cannot be empty if provided")
            }
        }
    }
}

impl std::error::Error for ValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_error_display_invalid_commit_type() {
        let error = ValidationError::InvalidCommitType("invalid".to_string());
        let expected = "Invalid commit type: 'invalid'. Must be one of: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert";
        assert_eq!(error.to_string(), expected);
    }

    #[test]
    fn validation_error_display_empty_description() {
        let error = ValidationError::EmptyDescription;
        assert_eq!(error.to_string(), "Description cannot be empty");
    }

    #[test]
    fn validation_error_display_description_too_long() {
        let error = ValidationError::DescriptionTooLong(100);
        assert_eq!(
            error.to_string(),
            "Description is too long (100 characters). Maximum is 72 characters"
        );
    }

    #[test]
    fn validation_error_display_invalid_scope() {
        let error = ValidationError::InvalidScope("invalid!".to_string());
        assert_eq!(
            error.to_string(),
            "Invalid scope: 'invalid!'. Scope must be alphanumeric with hyphens/underscores"
        );
    }

    #[test]
    fn validation_error_display_empty_breaking_change() {
        let error = ValidationError::EmptyBreakingChange;
        assert_eq!(
            error.to_string(),
            "Breaking change description cannot be empty"
        );
    }

    #[test]
    fn validation_error_display_empty_body() {
        let error = ValidationError::EmptyBody;
        assert_eq!(error.to_string(), "Body cannot be empty if provided");
    }
}
