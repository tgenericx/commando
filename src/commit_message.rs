/// Commit Message Domain Model
///
/// Represents a structured commit message following conventional commits format.
/// All validation happens at construction time, making invalid states unrepresentable.

#[derive(Debug, Clone, PartialEq)]
pub struct CommitMessage {
    commit_type: CommitType,
    scope: Option<String>,
    description: String,
    body: Option<String>,
    breaking_change: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommitType {
    Feat,
    Fix,
    Docs,
    Style,
    Refactor,
    Perf,
    Test,
    Build,
    Ci,
    Chore,
    Revert,
}

impl CommitType {
    /// Returns the string representation of the commit type
    pub fn as_str(&self) -> &'static str {
        match self {
            CommitType::Feat => "feat",
            CommitType::Fix => "fix",
            CommitType::Docs => "docs",
            CommitType::Style => "style",
            CommitType::Refactor => "refactor",
            CommitType::Perf => "perf",
            CommitType::Test => "test",
            CommitType::Build => "build",
            CommitType::Ci => "ci",
            CommitType::Chore => "chore",
            CommitType::Revert => "revert",
        }
    }

    /// Parse a commit type from a string
    pub fn from_str(s: &str) -> Result<Self, ValidationError> {
        match s.to_lowercase().as_str() {
            "feat" => Ok(CommitType::Feat),
            "fix" => Ok(CommitType::Fix),
            "docs" => Ok(CommitType::Docs),
            "style" => Ok(CommitType::Style),
            "refactor" => Ok(CommitType::Refactor),
            "perf" => Ok(CommitType::Perf),
            "test" => Ok(CommitType::Test),
            "build" => Ok(CommitType::Build),
            "ci" => Ok(CommitType::Ci),
            "chore" => Ok(CommitType::Chore),
            "revert" => Ok(CommitType::Revert),
            _ => Err(ValidationError::InvalidCommitType(s.to_string())),
        }
    }
}

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
                write!(
                    f,
                    "Invalid commit type: '{}'. Must be one of: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert",
                    t
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

impl CommitMessage {
    /// Creates a new commit message with validation
    ///
    /// # Arguments
    /// * `commit_type` - The type of commit (feat, fix, etc.)
    /// * `scope` - Optional scope (e.g., "parser", "api")
    /// * `description` - Short description (required, max 72 chars)
    /// * `body` - Optional detailed description
    /// * `breaking_change` - Optional breaking change description
    ///
    /// # Errors
    /// Returns `ValidationError` if any validation rules are violated
    pub fn new(
        commit_type: CommitType,
        scope: Option<String>,
        description: String,
        body: Option<String>,
        breaking_change: Option<String>,
    ) -> Result<Self, ValidationError> {
        // Validate description
        Self::validate_description(&description)?;

        // Validate scope if provided
        if let Some(ref s) = scope {
            Self::validate_scope(s)?;
        }

        // Validate body if provided - FIXED: collapsed if statement
        if let Some(ref b) = body
            && b.trim().is_empty()
        {
            return Err(ValidationError::EmptyBody);
        }

        // Validate breaking change if provided - FIXED: collapsed if statement
        if let Some(ref bc) = breaking_change
            && bc.trim().is_empty()
        {
            return Err(ValidationError::EmptyBreakingChange);
        }

        Ok(CommitMessage {
            commit_type,
            scope,
            description,
            body,
            breaking_change,
        })
    }

    fn validate_description(description: &str) -> Result<(), ValidationError> {
        let trimmed = description.trim();

        if trimmed.is_empty() {
            return Err(ValidationError::EmptyDescription);
        }

        if trimmed.len() > 72 {
            return Err(ValidationError::DescriptionTooLong(trimmed.len()));
        }

        Ok(())
    }

    pub fn validate_scope(scope: &str) -> Result<(), ValidationError> {
        let trimmed = scope.trim();

        if trimmed.is_empty() {
            return Err(ValidationError::InvalidScope(scope.to_string()));
        }

        // Scope should be alphanumeric with hyphens and underscores
        if !trimmed
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err(ValidationError::InvalidScope(scope.to_string()));
        }

        Ok(())
    }

    /// Renders the commit message as a conventional commit string
    pub fn to_conventional_commit(&self) -> String {
        let mut result = String::new();

        // Type and scope
        result.push_str(self.commit_type.as_str());
        if let Some(ref scope) = self.scope {
            result.push('(');
            result.push_str(scope);
            result.push(')');
        }

        // Breaking change indicator
        if self.breaking_change.is_some() {
            result.push('!');
        }

        // Description
        result.push_str(": ");
        result.push_str(&self.description);

        // Body
        if let Some(ref body) = self.body {
            result.push_str("\n\n");
            result.push_str(body);
        }

        // Breaking change footer
        if let Some(ref breaking) = self.breaking_change {
            result.push_str("\n\nBREAKING CHANGE: ");
            result.push_str(breaking);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_minimal_commit() {
        let commit = CommitMessage::new(
            CommitType::Feat,
            None,
            "add user authentication".to_string(),
            None,
            None,
        );
        assert!(commit.is_ok());
    }

    #[test]
    fn valid_commit_with_scope() {
        let commit = CommitMessage::new(
            CommitType::Fix,
            Some("parser".to_string()),
            "handle edge case in lexer".to_string(),
            None,
            None,
        );
        assert!(commit.is_ok());
    }

    #[test]
    fn valid_commit_with_body() {
        let commit = CommitMessage::new(
            CommitType::Feat,
            None,
            "add search feature".to_string(),
            Some("Implements full-text search using index".to_string()),
            None,
        );
        assert!(commit.is_ok());
    }

    #[test]
    fn valid_commit_with_breaking_change() {
        let commit = CommitMessage::new(
            CommitType::Feat,
            Some("api".to_string()),
            "change authentication flow".to_string(),
            None,
            Some("API endpoints now require OAuth tokens".to_string()),
        );
        assert!(commit.is_ok());
    }

    #[test]
    fn empty_description_fails() {
        let commit = CommitMessage::new(CommitType::Feat, None, "".to_string(), None, None);
        assert!(matches!(commit, Err(ValidationError::EmptyDescription)));
    }

    #[test]
    fn description_too_long_fails() {
        let long_desc = "a".repeat(73);
        let commit = CommitMessage::new(CommitType::Feat, None, long_desc, None, None);
        assert!(matches!(
            commit,
            Err(ValidationError::DescriptionTooLong(_))
        ));
    }

    #[test]
    fn invalid_scope_fails() {
        let commit = CommitMessage::new(
            CommitType::Feat,
            Some("invalid scope!".to_string()),
            "description".to_string(),
            None,
            None,
        );
        assert!(matches!(commit, Err(ValidationError::InvalidScope(_))));
    }

    #[test]
    fn empty_body_fails() {
        let commit = CommitMessage::new(
            CommitType::Feat,
            None,
            "description".to_string(),
            Some("   ".to_string()),
            None,
        );
        assert!(matches!(commit, Err(ValidationError::EmptyBody)));
    }

    #[test]
    fn empty_breaking_change_fails() {
        let commit = CommitMessage::new(
            CommitType::Feat,
            None,
            "description".to_string(),
            None,
            Some("".to_string()),
        );
        assert!(matches!(commit, Err(ValidationError::EmptyBreakingChange)));
    }

    #[test]
    fn commit_type_from_str_valid() {
        assert_eq!(CommitType::from_str("feat").unwrap(), CommitType::Feat);
        assert_eq!(CommitType::from_str("FEAT").unwrap(), CommitType::Feat);
        assert_eq!(CommitType::from_str("fix").unwrap(), CommitType::Fix);
    }

    #[test]
    fn commit_type_from_str_invalid() {
        assert!(CommitType::from_str("invalid").is_err());
        assert!(CommitType::from_str("feature").is_err());
    }

    #[test]
    fn to_conventional_commit_minimal() {
        let commit = CommitMessage::new(
            CommitType::Feat,
            None,
            "add feature".to_string(),
            None,
            None,
        )
        .unwrap();

        assert_eq!(commit.to_conventional_commit(), "feat: add feature");
    }

    #[test]
    fn to_conventional_commit_with_scope() {
        let commit = CommitMessage::new(
            CommitType::Fix,
            Some("parser".to_string()),
            "fix bug".to_string(),
            None,
            None,
        )
        .unwrap();

        assert_eq!(commit.to_conventional_commit(), "fix(parser): fix bug");
    }

    #[test]
    fn to_conventional_commit_with_body() {
        let commit = CommitMessage::new(
            CommitType::Feat,
            None,
            "add feature".to_string(),
            Some("This is the body".to_string()),
            None,
        )
        .unwrap();

        assert_eq!(
            commit.to_conventional_commit(),
            "feat: add feature\n\nThis is the body"
        );
    }

    #[test]
    fn to_conventional_commit_with_breaking_change() {
        let commit = CommitMessage::new(
            CommitType::Feat,
            Some("api".to_string()),
            "change endpoint".to_string(),
            None,
            Some("Removes v1 API".to_string()),
        )
        .unwrap();

        assert_eq!(
            commit.to_conventional_commit(),
            "feat(api)!: change endpoint\n\nBREAKING CHANGE: Removes v1 API"
        );
    }

    #[test]
    fn to_conventional_commit_full() {
        let commit = CommitMessage::new(
            CommitType::Feat,
            Some("auth".to_string()),
            "implement OAuth".to_string(),
            Some("Added OAuth 2.0 support with refresh tokens".to_string()),
            Some("Old session-based auth is removed".to_string()),
        )
        .unwrap();

        let expected = "feat(auth)!: implement OAuth\n\n\
                        Added OAuth 2.0 support with refresh tokens\n\n\
                        BREAKING CHANGE: Old session-based auth is removed";

        assert_eq!(commit.to_conventional_commit(), expected);
    }
}
