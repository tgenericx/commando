use super::{CommitResult, GitError};

pub trait IGitBackend: Send + Sync {
    fn has_staged_changes(&self) -> Result<bool, GitError>;
    fn commit(&self, msg: &str) -> Result<CommitResult, GitError>;
}
