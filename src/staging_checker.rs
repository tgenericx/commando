use std::process::Command;

#[derive(Default, Debug)]
pub struct StagingChecker;

impl StagingChecker {
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

    /// Check if there are any staged changes in the Git repository
    ///
    /// Uses `git diff --cached --name-only` to detect staged files.
    ///
    /// # Returns
    /// - `Ok(true)` if staged changes exist
    /// - `Ok(false)` if no staged changes
    /// - `Err(String)` if not in a Git repository or Git is not available
    pub fn has_staged_changes(&self) -> Result<bool, String> {
        if !self.is_git_repo() {
            return Err("Not inside a git repository".to_string());
        }

        let output = Command::new("git")
            .args(["diff", "--cached", "--name-only"])
            .output()
            .map_err(|e| format!("Failed to execute git command: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Git command failed: {}", stderr.trim()));
        }

        Ok(!output.stdout.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn staging_checker_can_be_created() {
        let _checker = StagingChecker::new();
    }

    #[test]
    fn staging_checker_has_default() {
        let _checker = StagingChecker::default();
    }

    #[test]
    fn has_staged_changes_returns_result() {
        let checker = StagingChecker::new();
        // This will succeed or fail depending on whether we're in a git repo
        let _result = checker.has_staged_changes();
    }
}
