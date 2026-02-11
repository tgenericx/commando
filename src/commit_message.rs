use crate::commit_types::CommitType;

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

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    InvalidCommitType(String),
    EmptyDescription,
    DescriptionTooLong(usize),
    InvalidScope(String),
    EmptyBreakingChange,
    BreakingChangeMismatch,
    EmptyBody,
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
            ValidationError::EmptyBody => write!(f, "Body cannot be empty if provided"),
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

impl CommitMessage {
    pub fn new(
        commit_type: CommitType,
        scope: Option<String>,
        description: String,
        body: Option<String>,
        breaking_change: Option<String>,
    ) -> Result<Self, ValidationError> {
        Self::validate_description(&description)?;

        if let Some(ref s) = scope {
            Self::validate_scope(s)?;
        }

        if let Some(ref b) = body
            && b.trim().is_empty()
        {
            return Err(ValidationError::EmptyBody);
        }

        if let Some(ref bc) = breaking_change
            && bc.trim().is_empty()
        {
            return Err(ValidationError::EmptyBreakingChange);
        }

        Ok(Self {
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

        if trimmed.is_empty()
            || !trimmed
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err(ValidationError::InvalidScope(scope.to_string()));
        }

        Ok(())
    }

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

    // Getters
    pub fn commit_type(&self) -> CommitType {
        self.commit_type
    }

    pub fn scope(&self) -> Option<&str> {
        self.scope.as_deref()
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }

    pub fn breaking_change(&self) -> Option<&str> {
        self.breaking_change.as_deref()
    }
}
