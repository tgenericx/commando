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
    fn extract_sha(output: &str) -> Option<String> {
        // Git commit output typically looks like:
        // [main 1234abc] commit message
        // or
        // [main (root-commit) 1234abc] commit message

        for line in output.lines() {
            if let (Some(bracket_start), Some(bracket_end)) = (line.find('['), line.find(']')) {
                let inside_brackets = &line[bracket_start + 1..bracket_end];

                // Extract the SHA (usually after the branch name)
                let parts: Vec<&str> = inside_brackets.split_whitespace().collect();

                // SHA is typically the last part, or second-to-last if root-commit
                for part in parts.iter().rev() {
                    // SHA is hex digits and typically 7-40 chars
                    if part.len() >= 7
                        && part.len() <= 40
                        && part.chars().all(|c| c.is_ascii_hexdigit())
                        && *part != "root"
                        && *part != "commit"
                    {
                        return Some(part.to_string());
                    }
                }
            }
        }
        None
    }

    fn extract_summary(output: &str) -> String {
        // Try to extract useful summary from git output
        let lines: Vec<&str> = output.lines().collect();

        if lines.is_empty() {
            return String::from("Commit created");
        }

        // Return first non-empty line with brackets removed
        for line in lines {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                // Remove the [branch sha] part
                if let Some(summary) = trimmed.split_once("] ") {
                    return summary.1.to_string();
                }
                return trimmed.to_string();
            }
        }

        String::from("Commit created")
    }

    fn from_output(output: &Output) -> Self {
        let stdout = String::from_utf8_lossy(&output.stdout);

        let sha = Self::extract_sha(&stdout).unwrap_or_else(|| "unknown".to_string());
        let summary = Self::extract_summary(&stdout);

        CommitResult { sha, summary }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_sha_from_git_output() {
        let output = "[exec-commit 1628fa6] feat: add Git commit execution";
        let sha = CommitResult::extract_sha(output).unwrap();
        assert_eq!(sha, "1628fa6");
    }

    #[test]
    fn parses_full_sha() {
        let output = "[main abc123def4567890123456789012345678901234] fix: handle edge case";
        let sha = CommitResult::extract_sha(output).unwrap();
        assert_eq!(sha, "abc123def4567890123456789012345678901234");
    }

    #[test]
    fn parses_root_commit() {
        let output = "[main (root-commit) abc1234] Initial commit";
        let sha = CommitResult::extract_sha(output).unwrap();
        assert_eq!(sha, "abc1234");
    }

    #[test]
    fn extracts_summary() {
        let output = "[feature 1234abc] feat: add new feature\n 1 file changed, 42 insertions(+)";
        let summary = CommitResult::extract_summary(output);
        assert_eq!(summary, "feat: add new feature");
    }

    #[test]
    fn handles_empty_output() {
        let output = "";
        let summary = CommitResult::extract_summary(output);
        assert_eq!(summary, "Commit created");
    }
}
