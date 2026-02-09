use crate::commit_message::{CommitMessage, CommitType, ValidationError};
use std::io::{self, Write};

/// Interactive input collector for commit messages
///
/// Guides users through creating a valid commit message step-by-step.
/// Validates each field individually and provides clear feedback.
pub struct InputCollector;

impl InputCollector {
    pub fn new() -> Self {
        Self
    }

    /// Collect all commit message data from the user interactively
    ///
    /// Returns a validated CommitMessage or an error if the user cancels.
    pub fn collect(&self) -> Result<CommitMessage, String> {
        println!("\n=== Create Commit Message ===\n");

        // Step 1: Collect type
        let commit_type = self.collect_type()?;

        // Step 2: Collect scope (optional)
        let scope = self.collect_scope()?;

        // Step 3: Collect description
        let description = self.collect_description()?;

        // Step 4: Collect body (optional)
        let body = self.collect_body()?;

        // Step 5: Collect breaking change (optional)
        let breaking_change = self.collect_breaking_change()?;

        // Create commit message (should always succeed since we validated each field)
        CommitMessage::new(commit_type, scope, description, body, breaking_change)
            .map_err(|e| format!("Unexpected validation error: {}", e))
    }

    fn collect_type(&self) -> Result<CommitType, String> {
        println!("1. What type of change is this?");
        println!("   · feat      (new feature)");
        println!("   · fix       (bug fix)");
        println!("   · docs      (documentation)");
        println!("   · style     (formatting, missing semicolons, etc.)");
        println!("   · refactor  (code restructuring)");
        println!("   · perf      (performance improvement)");
        println!("   · test      (adding tests)");
        println!("   · build     (build system changes)");
        println!("   · ci        (CI configuration)");
        println!("   · chore     (maintenance)");
        println!("   · revert    (revert previous commit)");
        println!();

        loop {
            print!("Your choice: ");
            io::stdout().flush().map_err(|e| e.to_string())?;

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| e.to_string())?;
            let input = input.trim();

            match CommitType::from_str(input) {
                Ok(commit_type) => {
                    println!();
                    return Ok(commit_type);
                }
                Err(ValidationError::InvalidCommitType(_)) => {
                    println!(
                        "Error: '{}' is not a valid type. Please choose from the list above.",
                        input
                    );
                    print!("Your choice: ");
                    io::stdout().flush().map_err(|e| e.to_string())?;
                }
                Err(e) => return Err(e.to_string()),
            }
        }
    }

    fn collect_scope(&self) -> Result<Option<String>, String> {
        println!("2. Which module/area does this affect?");
        println!("   (e.g., api, parser, auth-service)");
        println!("   Press Enter to skip");
        println!();

        loop {
            print!("Scope: ");
            io::stdout().flush().map_err(|e| e.to_string())?;

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| e.to_string())?;
            let input = input.trim();

            if input.is_empty() {
                println!();
                return Ok(None);
            }

            // Validate scope
            if input
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
            {
                println!();
                return Ok(Some(input.to_string()));
            } else {
                println!("Error: Scope must be alphanumeric with hyphens/underscores only.");
                println!("Try again or press Enter to skip.");
            }
        }
    }

    fn collect_description(&self) -> Result<String, String> {
        println!("3. What does this change do?");
        println!("   (Brief description, max 72 characters)");
        println!();

        loop {
            print!("Description: ");
            io::stdout().flush().map_err(|e| e.to_string())?;

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| e.to_string())?;
            let input = input.trim().to_string();

            if input.is_empty() {
                println!("Error: Description cannot be empty.");
                continue;
            }

            if input.len() > 72 {
                println!(
                    "Error: Description is too long ({} chars). Maximum is 72 characters.",
                    input.len()
                );
                println!("Please shorten your description.");
                continue;
            }

            println!();
            return Ok(input);
        }
    }

    fn collect_body(&self) -> Result<Option<String>, String> {
        println!("4. Would you like to add more details? (y/N)");
        print!("Choice: ");
        io::stdout().flush().map_err(|e| e.to_string())?;

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .map_err(|e| e.to_string())?;
        let choice = choice.trim().to_lowercase();

        if choice != "y" && choice != "yes" {
            println!();
            return Ok(None);
        }

        println!();
        println!("Enter detailed description (press Ctrl+D when done, Ctrl+C to cancel):");
        println!();

        let mut body = String::new();
        loop {
            let mut line = String::new();
            match io::stdin().read_line(&mut line) {
                Ok(0) => break, // EOF (Ctrl+D)
                Ok(_) => body.push_str(&line),
                Err(e) => return Err(e.to_string()),
            }
        }

        let body = body.trim().to_string();
        if body.is_empty() {
            println!();
            return Ok(None);
        }

        println!();
        Ok(Some(body))
    }

    fn collect_breaking_change(&self) -> Result<Option<String>, String> {
        println!("5. Does this change break existing functionality? (y/N)");
        print!("Choice: ");
        io::stdout().flush().map_err(|e| e.to_string())?;

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .map_err(|e| e.to_string())?;
        let choice = choice.trim().to_lowercase();

        if choice != "y" && choice != "yes" {
            println!();
            return Ok(None);
        }

        println!();
        println!("Describe what breaks and how users should adapt:");
        println!();

        loop {
            print!("Breaking change: ");
            io::stdout().flush().map_err(|e| e.to_string())?;

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| e.to_string())?;
            let input = input.trim().to_string();

            if input.is_empty() {
                println!("Error: Breaking change description cannot be empty.");
                println!("Press Ctrl+C to cancel or provide a description.");
                continue;
            }

            println!();
            return Ok(Some(input));
        }
    }
}

impl Default for InputCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_collector_can_be_created() {
        let _collector = InputCollector::new();
    }

    #[test]
    fn input_collector_has_default() {
        let _collector = InputCollector::default();
    }
}
