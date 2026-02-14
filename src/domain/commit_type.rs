/// Commit Type Domain Model
///
/// Represents the type of a conventional commit.
/// All validation happens at construction time, making invalid states unrepresentable.
use crate::domain::error::ValidationError;

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

    /// Returns all valid commit types as a slice of strings
    pub fn all_as_str() -> &'static [&'static str] {
        &[
            "feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "ci", "chore",
            "revert",
        ]
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

impl std::fmt::Display for CommitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commit_type_from_str_valid() {
        assert_eq!(CommitType::from_str("feat").unwrap(), CommitType::Feat);
        assert_eq!(CommitType::from_str("FEAT").unwrap(), CommitType::Feat);
        assert_eq!(CommitType::from_str("fix").unwrap(), CommitType::Fix);
        assert_eq!(CommitType::from_str("docs").unwrap(), CommitType::Docs);
        assert_eq!(CommitType::from_str("style").unwrap(), CommitType::Style);
        assert_eq!(
            CommitType::from_str("refactor").unwrap(),
            CommitType::Refactor
        );
        assert_eq!(CommitType::from_str("perf").unwrap(), CommitType::Perf);
        assert_eq!(CommitType::from_str("test").unwrap(), CommitType::Test);
        assert_eq!(CommitType::from_str("build").unwrap(), CommitType::Build);
        assert_eq!(CommitType::from_str("ci").unwrap(), CommitType::Ci);
        assert_eq!(CommitType::from_str("chore").unwrap(), CommitType::Chore);
        assert_eq!(CommitType::from_str("revert").unwrap(), CommitType::Revert);
    }

    #[test]
    fn commit_type_from_str_invalid() {
        assert!(matches!(
            CommitType::from_str("invalid"),
            Err(ValidationError::InvalidCommitType(_))
        ));
        assert!(matches!(
            CommitType::from_str("feature"),
            Err(ValidationError::InvalidCommitType(_))
        ));
    }

    #[test]
    fn commit_type_as_str() {
        assert_eq!(CommitType::Feat.as_str(), "feat");
        assert_eq!(CommitType::Fix.as_str(), "fix");
        assert_eq!(CommitType::Docs.as_str(), "docs");
    }

    #[test]
    fn commit_type_display() {
        assert_eq!(format!("{}", CommitType::Feat), "feat");
        assert_eq!(format!("{}", CommitType::Fix), "fix");
    }

    #[test]
    fn commit_type_all_as_str() {
        let all = CommitType::all_as_str();
        assert_eq!(all.len(), 11);
        assert_eq!(
            all,
            &[
                "feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "ci", "chore",
                "revert"
            ]
        );
    }
}
