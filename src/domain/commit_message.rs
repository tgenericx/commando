/// Commit Message Domain Model
///
/// Represents a structured commit message following conventional commits format.
/// All validation happens at construction time, making invalid states unrepresentable.
use crate::domain::commit_type::CommitType;
use crate::domain::error::DomainError;

#[derive(Debug, Clone, PartialEq)]
pub struct CommitMessage {
    commit_type: CommitType,
    scope: Option<String>,
    description: String,
    body: Option<String>,
    breaking_change: Option<String>,
}

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
    /// Returns `DomainError` if any validation rules are violated
    pub fn new(
        commit_type: CommitType,
        scope: Option<String>,
        description: String,
        body: Option<String>,
        breaking_change: Option<String>,
    ) -> Result<Self, DomainError> {
        // Validate description
        Self::validate_description(&description)?;

        // Validate scope if provided
        if let Some(ref s) = scope {
            Self::validate_scope(s)?;
        }

        // Validate body if provided
        if let Some(ref b) = body
            && b.trim().is_empty()
        {
            return Err(DomainError::EmptyBody);
        }

        // Validate breaking change if provided
        if let Some(ref bc) = breaking_change
            && bc.trim().is_empty()
        {
            return Err(DomainError::EmptyBreakingChange);
        }

        Ok(CommitMessage {
            commit_type,
            scope,
            description,
            body,
            breaking_change,
        })
    }

    fn validate_description(description: &str) -> Result<(), DomainError> {
        let trimmed = description.trim();

        if trimmed.is_empty() {
            return Err(DomainError::EmptyDescription);
        }

        if trimmed.len() > 72 {
            return Err(DomainError::DescriptionTooLong(trimmed.len()));
        }

        Ok(())
    }

    pub fn validate_scope(scope: &str) -> Result<(), DomainError> {
        let trimmed = scope.trim();

        if trimmed.is_empty() {
            return Err(DomainError::InvalidScope(scope.to_string()));
        }

        // Scope should be alphanumeric with hyphens and underscores
        if !trimmed
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err(DomainError::InvalidScope(scope.to_string()));
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

impl std::fmt::Display for CommitMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_conventional_commit())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::commit_type::CommitType;
    use crate::domain::error::DomainError;

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
        assert!(matches!(commit, Err(DomainError::EmptyDescription)));
    }

    #[test]
    fn description_too_long_fails() {
        let long_desc = "a".repeat(73);
        let commit = CommitMessage::new(CommitType::Feat, None, long_desc, None, None);
        assert!(matches!(commit, Err(DomainError::DescriptionTooLong(_))));
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
        assert!(matches!(commit, Err(DomainError::InvalidScope(_))));
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
        assert!(matches!(commit, Err(DomainError::EmptyBody)));
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
        assert!(matches!(commit, Err(DomainError::EmptyBreakingChange)));
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

    #[test]
    fn display_implementation() {
        let commit = CommitMessage::new(
            CommitType::Feat,
            Some("auth".to_string()),
            "implement OAuth".to_string(),
            None,
            None,
        )
        .unwrap();

        assert_eq!(format!("{}", commit), "feat(auth): implement OAuth");
    }
}
