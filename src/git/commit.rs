use std::process::{Command, Output};

use crate::cli::CliError;

pub struct CommitExecutor;

impl CommitExecutor {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, message: &str) -> Result<CommitResult, CliError> {
        // Use git commit -m with proper escaping
        let output = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(message)
            .output()
            .map_err(|e| CliError::GitError(e.to_string()))?;

        if output.status.success() {
            CommitResult::from_output(&output)
        } else {
            Err(CliError::GitError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ))
        }
    }
}

#[derive(Debug)]
pub struct CommitResult {
    pub sha: String,
    pub summary: String,
}

impl CommitResult {
    fn from_output(output: &Output) -> Result<Self, CliError> {
        let stdout = String::from_utf8_lossy(&output.stdout);

        // Parse git output to extract SHA
        // Format: "[main (root-commit) abc1234] feat: add login"
        let sha = stdout
            .split_whitespace()
            .find(|word| word.len() == 40 || word.len() == 7)
            .unwrap_or("unknown")
            .to_string();

        let summary = stdout
            .lines()
            .next()
            .unwrap_or("Commit successful")
            .to_string();

        Ok(CommitResult { sha, summary })
    }
}

impl Default for CommitExecutor {
    fn default() -> Self {
        Self::new()
    }
}
