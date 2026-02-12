use std::process::Command;

use crate::{cli::CliError, git::CommitResult};

#[derive(Debug, Default)]
pub struct CommitExecutor;

impl CommitExecutor {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, message: &str) -> Result<CommitResult, CliError> {
        let commit_output = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(message)
            .output()
            .map_err(|e| CliError::GitError(e.to_string()))?;

        if !commit_output.status.success() {
            return Err(CliError::GitError(
                String::from_utf8_lossy(&commit_output.stderr).to_string(),
            ));
        }

        let sha_output = Command::new("git").arg("rev-parse").arg("HEAD").output()?;
        let sha = String::from_utf8_lossy(&sha_output.stdout)
            .trim()
            .to_string();

        let summary = message.lines().next().unwrap_or("").to_string();

        Ok(CommitResult { sha, summary })
    }
}
