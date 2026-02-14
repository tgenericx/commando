use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum GitError {
    NotAGitRepository,
    ExecutionFailed(String),
}

impl fmt::Display for GitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitError::NotAGitRepository => write!(f, "Not a git repository"),
            GitError::ExecutionFailed(msg) => write!(f, "Git execution failed: {}", msg),
        }
    }
}

impl std::error::Error for GitError {}
