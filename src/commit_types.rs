use crate::validation::ValidationError;

/// Shared commit type definitions for the entire application
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    pub const ALL_TYPES: [Self; 11] = [
        Self::Feat,
        Self::Fix,
        Self::Docs,
        Self::Style,
        Self::Refactor,
        Self::Perf,
        Self::Test,
        Self::Build,
        Self::Ci,
        Self::Chore,
        Self::Revert,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Feat => "feat",
            Self::Fix => "fix",
            Self::Docs => "docs",
            Self::Style => "style",
            Self::Refactor => "refactor",
            Self::Perf => "perf",
            Self::Test => "test",
            Self::Build => "build",
            Self::Ci => "ci",
            Self::Chore => "chore",
            Self::Revert => "revert",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, ValidationError> {
        match s.trim().to_lowercase().as_str() {
            "feat" => Ok(Self::Feat),
            "fix" => Ok(Self::Fix),
            "docs" => Ok(Self::Docs),
            "style" => Ok(Self::Style),
            "refactor" => Ok(Self::Refactor),
            "perf" => Ok(Self::Perf),
            "test" => Ok(Self::Test),
            "build" => Ok(Self::Build),
            "ci" => Ok(Self::Ci),
            "chore" => Ok(Self::Chore),
            "revert" => Ok(Self::Revert),
            _ => Err(ValidationError::InvalidCommitType(s.trim().to_string())),
        }
    }

    pub fn all_as_str() -> Vec<&'static str> {
        Self::ALL_TYPES.iter().map(|t| t.as_str()).collect()
    }
}
