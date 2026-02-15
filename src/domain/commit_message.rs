/// Commit Message Domain Model
use crate::domain::commit_type::CommitType;
use crate::domain::error::DomainError;

#[derive(Debug, Clone, PartialEq)]
pub struct CommitMessage {
    commit_type: CommitType,
    scope: Option<String>,
    description: String,
    body: Option<String>,
    breaking_change: Option<String>,
    /// All footers except BREAKING CHANGE, in order of appearance.
    /// e.g. [("Refs", "#42"), ("Co-authored-by", "Name <email>")]
    footers: Vec<(String, String)>,
}

impl CommitMessage {
    pub fn new(
        commit_type: CommitType,
        scope: Option<String>,
        description: String,
        body: Option<String>,
        breaking_change: Option<String>,
        footers: Vec<(String, String)>,
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
            footers,
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

    /// Renders the commit message as a conventional commit string.
    ///
    /// Footer ordering: BREAKING CHANGE (if present) first, then all other
    /// footers in their original order.
    pub fn to_conventional_commit(&self) -> String {
        let mut result = String::new();

        // Header
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

        // Body
        if let Some(ref body) = self.body {
            result.push_str("\n\n");
            result.push_str(body);
        }

        // Footer section — only open if there is at least one footer
        let has_footers = self.breaking_change.is_some() || !self.footers.is_empty();
        if has_footers {
            result.push_str("\n\n");

            if let Some(ref bc) = self.breaking_change {
                result.push_str("BREAKING CHANGE: ");
                result.push_str(bc);
                if !self.footers.is_empty() {
                    result.push('\n');
                }
            }

            for (i, (key, value)) in self.footers.iter().enumerate() {
                result.push_str(key);
                result.push_str(": ");
                result.push_str(value);
                if i < self.footers.len() - 1 {
                    result.push('\n');
                }
            }
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
/// BREAKING CHANGE footer → breaking_change field (drives the '!' marker).
/// All other footers → footers field, in order of appearance.
impl TryFrom<crate::compiler::CommitAst> for CommitMessage {
    type Error = DomainError;

    fn try_from(ast: crate::compiler::CommitAst) -> Result<Self, DomainError> {
        let commit_type = CommitType::from_str(&ast.header.commit_type)?;

        let breaking_change = ast
            .footers
            .iter()
            .find(|f| f.key == "BREAKING CHANGE" || f.key == "BREAKING-CHANGE")
            .map(|f| f.value.clone());

        let footers: Vec<(String, String)> = ast
            .footers
            .into_iter()
            .filter(|f| f.key != "BREAKING CHANGE" && f.key != "BREAKING-CHANGE")
            .map(|f| (f.key, f.value))
            .collect();

        CommitMessage::new(
            commit_type,
            ast.header.scope,
            ast.header.description,
            ast.body.map(|b| b.content),
            breaking_change,
            footers,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::commit_type::CommitType;
    use crate::domain::error::DomainError;

    fn no_footers() -> Vec<(String, String)> {
        vec![]
    }

    #[test]
    fn valid_minimal_commit() {
        assert!(
            CommitMessage::new(
                CommitType::Feat,
                None,
                "add login".into(),
                None,
                None,
                no_footers()
            )
            .is_ok()
        );
    }

    #[test]
    fn empty_description_fails() {
        assert!(matches!(
            CommitMessage::new(CommitType::Feat, None, "".into(), None, None, no_footers()),
            Err(DomainError::EmptyDescription)
        ));
    }

    #[test]
    fn description_too_long_fails() {
        assert!(matches!(
            CommitMessage::new(
                CommitType::Feat,
                None,
                "a".repeat(73),
                None,
                None,
                no_footers()
            ),
            Err(DomainError::DescriptionTooLong(_))
        ));
    }

    #[test]
    fn invalid_scope_fails() {
        assert!(matches!(
            CommitMessage::new(
                CommitType::Feat,
                Some("bad scope!".into()),
                "desc".into(),
                None,
                None,
                no_footers()
            ),
            Err(DomainError::InvalidScope(_))
        ));
    }

    #[test]
    fn empty_body_fails() {
        assert!(matches!(
            CommitMessage::new(
                CommitType::Feat,
                None,
                "desc".into(),
                Some("  ".into()),
                None,
                no_footers()
            ),
            Err(DomainError::EmptyBody)
        ));
    }

    #[test]
    fn empty_breaking_change_fails() {
        assert!(matches!(
            CommitMessage::new(
                CommitType::Feat,
                None,
                "desc".into(),
                None,
                Some("".into()),
                no_footers()
            ),
            Err(DomainError::EmptyBreakingChange)
        ));
    }

    #[test]
    fn renders_minimal() {
        let msg = CommitMessage::new(
            CommitType::Feat,
            None,
            "add feature".into(),
            None,
            None,
            no_footers(),
        )
        .unwrap();
        assert_eq!(msg.to_conventional_commit(), "feat: add feature");
    }

    #[test]
    fn renders_with_scope() {
        let msg = CommitMessage::new(
            CommitType::Fix,
            Some("parser".into()),
            "fix bug".into(),
            None,
            None,
            no_footers(),
        )
        .unwrap();
        assert_eq!(msg.to_conventional_commit(), "fix(parser): fix bug");
    }

    #[test]
    fn renders_with_body() {
        let msg = CommitMessage::new(
            CommitType::Feat,
            None,
            "add feature".into(),
            Some("This is the body".into()),
            None,
            no_footers(),
        )
        .unwrap();
        assert_eq!(
            msg.to_conventional_commit(),
            "feat: add feature\n\nThis is the body"
        );
    }

    #[test]
    fn renders_with_breaking_change_only() {
        let msg = CommitMessage::new(
            CommitType::Feat,
            Some("api".into()),
            "change endpoint".into(),
            None,
            Some("Removes v1 API".into()),
            no_footers(),
        )
        .unwrap();
        assert_eq!(
            msg.to_conventional_commit(),
            "feat(api)!: change endpoint\n\nBREAKING CHANGE: Removes v1 API"
        );
    }

    #[test]
    fn renders_refs_footer() {
        let msg = CommitMessage::new(
            CommitType::Fix,
            None,
            "patch null pointer".into(),
            None,
            None,
            vec![("Refs".into(), "#42".into())],
        )
        .unwrap();
        assert_eq!(
            msg.to_conventional_commit(),
            "fix: patch null pointer\n\nRefs: #42"
        );
    }

    #[test]
    fn renders_multiple_footers_in_order() {
        let msg = CommitMessage::new(
            CommitType::Fix,
            None,
            "patch thing".into(),
            None,
            None,
            vec![
                ("Refs".into(), "#42".into()),
                ("Closes".into(), "#99".into()),
            ],
        )
        .unwrap();
        let out = msg.to_conventional_commit();
        assert!(out.find("Refs:").unwrap() < out.find("Closes:").unwrap());
    }

    #[test]
    fn renders_breaking_change_before_other_footers() {
        let msg = CommitMessage::new(
            CommitType::Feat,
            Some("api".into()),
            "redesign".into(),
            None,
            Some("v1 removed".into()),
            vec![("Refs".into(), "#88".into())],
        )
        .unwrap();
        let out = msg.to_conventional_commit();
        assert!(out.find("BREAKING CHANGE:").unwrap() < out.find("Refs:").unwrap());
    }

    #[test]
    fn renders_full_commit() {
        let msg = CommitMessage::new(
            CommitType::Feat,
            Some("auth".into()),
            "implement OAuth".into(),
            Some("Added OAuth 2.0 support".into()),
            Some("Old sessions removed".into()),
            vec![("Refs".into(), "#142".into())],
        )
        .unwrap();
        let expected = "feat(auth)!: implement OAuth\n\n\
                        Added OAuth 2.0 support\n\n\
                        BREAKING CHANGE: Old sessions removed\n\
                        Refs: #142";
        assert_eq!(msg.to_conventional_commit(), expected);
    }

    // ── TryFrom<CommitAst> ────────────────────────────────────────────────────

    #[test]
    fn try_from_ast_invalid_type_is_domain_error() {
        use crate::compiler::CompilerPipeline;
        let ast = CompilerPipeline::new()
            .compile("notavalidtype: do something")
            .unwrap();
        assert!(matches!(
            CommitMessage::try_from(ast),
            Err(DomainError::InvalidCommitType(_))
        ));
    }
}
