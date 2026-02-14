//! Git-based implementation of the CommitExecutor and DryRunner ports

use std::process::Command;

use crate::domain::error::DomainError;
use crate::ports::{CommitExecutor, CommitResult, DryRunner};

#[derive(Debug, Default)]
pub struct GitCommitExecutor;

impl GitCommitExecutor {
    pub fn new() -> Self {
        Self
    }
}

impl CommitExecutor for GitCommitExecutor {
    fn execute(&self, message: &str) -> Result<CommitResult, DomainError> {
        // Execute git commit with the message
        let commit_output = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(message)
            .output()
            .map_err(|e| {
                DomainError::Infrastructure(format!("Failed to execute git commit: {}", e))
            })?;

        if !commit_output.status.success() {
            let stderr = String::from_utf8_lossy(&commit_output.stderr);
            return Err(DomainError::Infrastructure(format!(
                "Git commit failed: {}",
                stderr.trim()
            )));
        }

        // Get the commit SHA of the new commit
        let sha_output = Command::new("git")
            .arg("rev-parse")
            .arg("HEAD")
            .output()
            .map_err(|e| DomainError::Infrastructure(format!("Failed to get commit SHA: {}", e)))?;

        if !sha_output.status.success() {
            return Err(DomainError::Infrastructure(
                "Failed to get commit SHA after successful commit".to_string(),
            ));
        }

        let sha = String::from_utf8_lossy(&sha_output.stdout)
            .trim()
            .to_string();

        // Use the first line of the commit message as summary
        let summary = message.lines().next().unwrap_or("").to_string();

        Ok(CommitResult { sha, summary })
    }
}

impl DryRunner for GitCommitExecutor {
    fn dry_run(&self, message: &str) -> Result<(), DomainError> {
        let output = Command::new("git")
            .args(["commit", "--dry-run", "-m", message])
            .output()
            .map_err(|e| {
                DomainError::Infrastructure(format!(
                    "Failed to execute git commit --dry-run: {}",
                    e
                ))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(DomainError(format!(
                "Git dry-run failed: {}",
                stderr.trim()
            )));
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
