//! Git-based implementation of the StagingChecker port

use std::process::Command;

use super::error::GitError;
use crate::ports::StagingChecker;

#[derive(Debug, Default, Clone, Copy)]
pub struct GitStagingChecker;

impl StagingChecker for GitStagingChecker {
    type Error = GitError;

    fn has_staged_changes(&self) -> Result<bool, Self::Error> {
        let is_repo_output = Command::new("git")
            .args(["rev-parse", "--is-inside-work-tree"])
            .output()
            .map_err(|e| GitError::ExecutionFailed(format!("Failed to run git: {}", e)))?;

        if !is_repo_output.status.success() {
            return Err(GitError::NotAGitRepository);
        }

        let output = Command::new("git")
            .args(["diff", "--cached", "--name-only"])
            .output()
            .map_err(|e| GitError::ExecutionFailed(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(GitError::ExecutionFailed(stderr.trim().to_string()));
        }

        Ok(!output.stdout.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn git_staging_checker_can_be_created() {
        let _checker = GitStagingChecker::default();
    }

    #[test]
    fn git_staging_checker_has_default() {
        let _checker = GitStagingChecker::default();
    }

    #[test]
    fn has_staged_changes_returns_result() {
        let checker = GitStagingChecker::default();
        // This will succeed or fail depending on whether we're in a git repo
        let _result = checker.has_staged_changes();
    }
}
