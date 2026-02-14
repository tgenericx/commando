//! Git-based implementation of the StagingChecker port

use std::process::Command;

use crate::domain::error::DomainError;
use crate::ports::StagingChecker;

#[derive(Debug, Default)]
pub struct GitStagingChecker;

impl GitStagingChecker {
    pub fn new() -> Self {
        Self
    }

    fn is_git_repo(&self) -> bool {
        Command::new("git")
            .args(["rev-parse", "--is-inside-work-tree"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

impl StagingChecker for GitStagingChecker {
    fn has_staged_changes(&self) -> Result<bool, DomainError> {
        if !self.is_git_repo() {
            return Err(DomainError::Infrastructure("Not inside a git repository".to_string()));
        }

        let output = Command::new("git")
            .args(["diff", "--cached", "--name-only"])
            .output()
            .map_err(|e| DomainError::Infrastructure(format!("Failed to execute git command: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(DomainError::Infrastructure(format!("Git command failed: {}", stderr.trim())));
        }

        Ok(!output.stdout.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn git_staging_checker_can_be_created() {
        let _checker = GitStagingChecker::new();
    }

    #[test]
    fn git_staging_checker_has_default() {
        let _checker = GitStagingChecker::default();
    }

    #[test]
    fn has_staged_changes_returns_result() {
        let checker = GitStagingChecker::new();
        // This will succeed or fail depending on whether we're in a git repo
        let _result = checker.has_staged_changes();
    }
}
