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
    pub fn new(
        commit_type: CommitType,
        scope: Option<String>,
        description: String,
        body: Option<String>,
        breaking_change: Option<String>,
    ) -> Result<Self, DomainError> {
        Self::validate_description(&description)?;

        if let Some(ref s) = scope {
            Self::validate_scope(s)?;
        }

        if let Some(ref b) = body
            && b.trim().is_empty()
        {
            return Err(DomainError::EmptyBody);
        }

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
        if !trimmed
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err(DomainError::InvalidScope(scope.to_string()));
        }
        Ok(())
    }

    pub fn to_conventional_commit(&self) -> String {
        let mut result = String::new();

        result.push_str(self.commit_type.as_str());
        if let Some(ref scope) = self.scope {
            result.push('(');
            result.push_str(scope);
            result.push(')');
        }
        if self.breaking_change.is_some() {
            result.push('!');
        }
        result.push_str(": ");
        result.push_str(&self.description);

        if let Some(ref body) = self.body {
            result.push_str("\n\n");
            result.push_str(body);
        }
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

/// Bridge from compiler output to domain.
///
/// CommitAst carries raw strings — the parser never calls CommitType::from_str().
/// This impl is where the domain validates those raw values:
///   - commit_type string → CommitType variant (or DomainError::InvalidCommitType)
///   - description length, scope charset, etc. via CommitMessage::new()
///
/// Breaking change: if the AST has a BREAKING CHANGE footer, use its value.
/// If the header had '!' but no BREAKING CHANGE footer, that's still a breaking
/// commit — the '!' is cosmetic in the header when the footer is absent.
impl TryFrom<crate::compiler::CommitAst> for CommitMessage {
    type Error = DomainError;

    fn try_from(ast: crate::compiler::CommitAst) -> Result<Self, DomainError> {
        let commit_type = CommitType::from_str(&ast.header.commit_type)?;

        let breaking_change = ast
            .footers
            .iter()
            .find(|f| f.key == "BREAKING CHANGE" || f.key == "BREAKING-CHANGE")
            .map(|f| f.value.clone());

        CommitMessage::new(
            commit_type,
            ast.header.scope,
            ast.header.description,
            ast.body.map(|b| b.content),
            breaking_change,
        )
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

    // ── TryFrom<CommitAst> tests ──────────────────────────────────────────────

    #[test]
    fn try_from_ast_minimal() {
        use crate::compiler::{CommitAst, CompilerPipeline};
        let ast = CompilerPipeline::new().compile("feat: add login").unwrap();
        let msg = CommitMessage::try_from(ast).unwrap();
        assert_eq!(msg.to_conventional_commit(), "feat: add login");
    }

    #[test]
    fn try_from_ast_with_scope_and_body() {
        use crate::compiler::CompilerPipeline;
        let input = "fix(auth): correct expiry\n\nSession tokens were not checked.";
        let ast = CompilerPipeline::new().compile(input).unwrap();
        let msg = CommitMessage::try_from(ast).unwrap();
        assert!(
            msg.to_conventional_commit()
                .starts_with("fix(auth): correct expiry")
        );
    }

    #[test]
    fn try_from_ast_with_breaking_change_footer() {
        use crate::compiler::CompilerPipeline;
        let input = "feat!: redesign API\n\nBREAKING CHANGE: all v1 endpoints removed";
        let ast = CompilerPipeline::new().compile(input).unwrap();
        let msg = CommitMessage::try_from(ast).unwrap();
        assert!(msg.to_conventional_commit().contains("BREAKING CHANGE:"));
    }

    #[test]
    fn try_from_ast_invalid_type_returns_domain_error() {
        use crate::compiler::CompilerPipeline;
        let ast = CompilerPipeline::new()
            .compile("notatype: do something")
            .unwrap();
        let result = CommitMessage::try_from(ast);
        assert!(matches!(result, Err(DomainError::InvalidCommitType(_))));
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
