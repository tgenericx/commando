//! Git-based implementation of the CommitExecutor and DryRunner ports

use std::process::Command;

use super::error::GitError;
use crate::ports::{CommitExecutor, CommitResult, DryRunner};

#[derive(Debug, Default)]
pub struct GitCommitExecutor;

impl CommitExecutor for GitCommitExecutor {
    type Error = GitError;

    fn execute(&self, message: &str) -> Result<CommitResult, Self::Error> {
        let commit_output = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(message)
            .output()
            .map_err(|e| GitError::ExecutionFailed(e.to_string()))?;

        if !commit_output.status.success() {
            let stderr = String::from_utf8_lossy(&commit_output.stderr);
            return Err(GitError::ExecutionFailed(stderr.trim().to_string()));
        }

        let sha_output = Command::new("git")
            .arg("rev-parse")
            .arg("HEAD")
            .output()
            .map_err(|e| GitError::ExecutionFailed(e.to_string()))?;

        if !sha_output.status.success() {
            return Err(GitError::ExecutionFailed(
                "Failed to get commit SHA".to_string(),
            ));
        }

        let sha = String::from_utf8_lossy(&sha_output.stdout)
            .trim()
            .to_string();

        let summary = message.lines().next().unwrap_or("").to_string();

        Ok(CommitResult { sha, summary })
    }
}

impl DryRunner for GitCommitExecutor {
    type Error = GitError;

    fn dry_run(&self, message: &str) -> Result<(), Self::Error> {
        let output = Command::new("git")
            .args(["commit", "--dry-run", "-m", message])
            .output()
            .map_err(|e| GitError::ExecutionFailed(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(GitError::ExecutionFailed(stderr.trim().to_string()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn git_executor_can_be_created() {
        let _executor = GitCommitExecutor::new();
    }

    #[test]
    fn git_executor_has_default() {
        let _executor = GitCommitExecutor::default();
    }
}
