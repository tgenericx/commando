use std::process::Command;

/// Executes Git commits with the provided commit message
///
/// Handles Git command execution and provides clear error reporting.
#[derive(Default, Debug)]
pub struct CommitExecutor;

#[derive(Debug)]
pub struct CommitResult {
    pub sha: String,
    pub summary: String,
}

impl CommitExecutor {
    pub fn new() -> Self {
        Self
    }

    /// Execute a Git commit with the provided message
    ///
    /// # Arguments
    /// * `message` - The commit message to use
    ///
    /// # Returns
    /// * `Ok(CommitResult)` - Commit succeeded with SHA and summary
    /// * `Err(String)` - Commit failed with error message
    pub fn execute(&self, message: &str) -> Result<CommitResult, String> {
        // Execute git commit with the message
        let output = Command::new("git")
            .args(["commit", "-m", message])
            .output()
            .map_err(|e| format!("Failed to execute git commit: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Git commit failed: {}", stderr.trim()));
        }

        // Parse the output to extract commit SHA
        let stdout = String::from_utf8_lossy(&output.stdout);

        // Try to extract SHA from output (usually first line contains it)
        let sha = self.extract_sha(&stdout)?;
        let summary = self.extract_summary(&stdout);

        Ok(CommitResult { sha, summary })
    }

    /// Execute a dry-run to validate the commit would succeed
    ///
    /// # Arguments
    /// * `message` - The commit message to validate
    ///
    /// # Returns
    /// * `Ok(())` - Dry-run succeeded, commit would work
    /// * `Err(String)` - Dry-run failed with error message
    pub fn dry_run(&self, message: &str) -> Result<(), String> {
        let output = Command::new("git")
            .args(["commit", "--dry-run", "-m", message])
            .output()
            .map_err(|e| format!("Failed to execute git commit --dry-run: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Git dry-run failed: {}", stderr.trim()));
        }

        Ok(())
    }

    /// Get the current HEAD commit SHA (for reference)
    fn get_current_head(&self) -> Result<String, String> {
        let output = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .map_err(|e| format!("Failed to get current HEAD: {}", e))?;

        if !output.status.success() {
            return Err("Could not determine current HEAD".to_string());
        }

        let sha = String::from_utf8_lossy(&output.stdout).trim().to_string();

        Ok(sha)
    }

    fn extract_sha(&self, output: &str) -> Result<String, String> {
        // Git commit output typically looks like:
        // [main 1234abc] commit message
        // or
        // [main (root-commit) 1234abc] commit message

        for line in output.lines() {
            if let Some(bracket_start) = line.find('[')
                && let Some(bracket_end) = line.find(']')
            {
                let inside_brackets = &line[bracket_start + 1..bracket_end];

                // Extract the SHA (usually after the branch name)
                let parts: Vec<&str> = inside_brackets.split_whitespace().collect();

                // SHA is typically the last part, or second-to-last if root-commit
                for part in parts.iter().rev() {
                    // SHA is alphanumeric and typically 7-40 chars
                    if part.len() >= 7
                        && part.chars().all(|c| c.is_ascii_alphanumeric())
                        && *part != "root"
                    {
                        return Ok(part.to_string());
                    }
                }
            }
        }

        // Fallback: try to get current HEAD
        self.get_current_head()
            .or_else(|_| Ok("unknown".to_string()))
    }

    fn extract_summary(&self, output: &str) -> String {
        // Try to extract useful summary from git output
        let lines: Vec<&str> = output.lines().collect();

        if lines.is_empty() {
            return String::from("Commit created");
        }

        // Return first non-empty line as summary
        for line in lines {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }

        String::from("Commit created")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commit_executor_can_be_created() {
        let _executor = CommitExecutor::new();
    }

    #[test]
    fn commit_executor_has_default() {
        let _executor = CommitExecutor::default();
    }

    #[test]
    fn extract_sha_from_typical_output() {
        let executor = CommitExecutor::new();
        let output = "[main 1234abc] feat: add feature\n 1 file changed, 10 insertions(+)";

        let sha = executor.extract_sha(output).unwrap();
        assert_eq!(sha, "1234abc");
    }

    #[test]
    fn extract_sha_from_root_commit_output() {
        let executor = CommitExecutor::new();
        let output = "[main (root-commit) abc1234] Initial commit\n 1 file changed, 1 insertion(+)";

        let sha = executor.extract_sha(output).unwrap();
        assert_eq!(sha, "abc1234");
    }

    #[test]
    fn extract_summary_from_output() {
        let executor = CommitExecutor::new();
        let output = "[main 1234abc] feat: add feature\n 1 file changed, 10 insertions(+)";

        let summary = executor.extract_summary(output);
        assert!(summary.contains("main"));
    }
}
