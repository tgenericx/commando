use std::process::{Command, Output};

use crate::cli::CliError;

#[derive(Debug, Default)]
pub struct CommitExecutor;

impl CommitExecutor {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, message: &str) -> Result<CommitResult, CliError> {
        let output = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(message)
            .output()
            .map_err(|e| CliError::GitError(e.to_string()))?;

        if output.status.success() {
            Ok(CommitResult::from_output(&output))
        } else {
            Err(CliError::GitError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ))
        }
    }
}

#[derive(Debug, Default)]
pub struct CommitResult {
    pub sha: String,
    pub summary: String,
}

impl CommitResult {
    fn from_output(output: &Output) -> Self {
        let stdout = String::from_utf8_lossy(&output.stdout);

        // Git output format: "[branch (commit) sha] summary"
        // Example: "[exec-commit 1628fa6] feat: add Git commit execution"
        let line = stdout.lines().next().unwrap_or("");

        // Extract SHA between brackets
        let sha = line
            .split_whitespace()
            .find(|word| {
                // Git SHAs are hex digits, 7-40 chars long
                word.len() >= 7 && word.len() <= 40 && word.chars().all(|c| c.is_ascii_hexdigit())
            })
            .unwrap_or("unknown")
            .to_string();

        // Extract summary (everything after '] ')
        let summary = line
            .split_once("] ")
            .map(|(_, s)| s.to_string())
            .unwrap_or_else(|| line.to_string());

        CommitResult { sha, summary }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_sha_from_git_output() {
        let output = "[exec-commit 1628fa6] feat: add Git commit execution".to_string();
        let mock_output = std::process::Output {
            stdout: output.into_bytes(),
            stderr: vec![],
            status: std::process::ExitStatus::default(),
        };

        let result = CommitResult::from_output(&mock_output);
        assert_eq!(result.sha, "1628fa6");
        assert_eq!(result.summary, "feat: add Git commit execution");
    }

    #[test]
    fn parses_full_sha() {
        let output = "[main abc123def4567890] fix: handle edge case".to_string();
        let mock_output = std::process::Output {
            stdout: output.into_bytes(),
            stderr: vec![],
            status: std::process::ExitStatus::default(),
        };

        let result = CommitResult::from_output(&mock_output);
        assert_eq!(result.sha, "abc123def4567890");
    }
}
