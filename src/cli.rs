use crate::commit_executor::CommitExecutor;
use crate::commit_message::CommitMessage;
use crate::commit_types::CommitType;
use crate::input_collector::{CommitData, InputCollector};
use crate::staging_checker::StagingChecker;
use std::io::{self, Write};
use std::process::ExitCode;

/// CLI execution mode
#[derive(Debug, Clone, PartialEq)]
pub enum CliMode {
    /// Interactive mode: prompts user for all fields
    Interactive,
    /// Direct mode: commit message provided via command line
    Direct { message: String },
}

pub struct CliController {
    staging_checker: StagingChecker,
    input_collector: InputCollector,
    commit_executor: CommitExecutor,
    mode: CliMode,
}

impl CliController {
    /// Create a new CLI controller in interactive mode
    pub fn new() -> Self {
        Self {
            staging_checker: StagingChecker::new(),
            input_collector: InputCollector::new(),
            commit_executor: CommitExecutor::new(),
            mode: CliMode::Interactive,
        }
    }

    /// Create a new CLI controller with a specific mode
    pub fn with_mode(mode: CliMode) -> Self {
        Self {
            staging_checker: StagingChecker::new(),
            input_collector: InputCollector::new(),
            commit_executor: CommitExecutor::new(),
            mode,
        }
    }

    pub fn run(&self) -> ExitCode {
        match &self.mode {
            CliMode::Interactive => self.run_interactive(),
            CliMode::Direct { message } => self.run_direct(message),
        }
    }

    /// Run in direct mode with a pre-composed message
    fn run_direct(&self, message: &str) -> ExitCode {
        println!("Running in direct mode");
        println!();

        // Step 1: Check for staged changes
        println!("Checking for staged changes...");
        match self.staging_checker.has_staged_changes() {
            Ok(true) => {
                println!("✓ Staged changes detected");
                println!();
            }
            Ok(false) => {
                println!("✗ No staged changes found");
                println!();
                println!("Please stage your changes first:");
                println!("  git add <files>");
                return ExitCode::FAILURE;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                return ExitCode::FAILURE;
            }
        }

        // Step 2: Parse the direct message
        let commit_message = match self.parse_direct_message(message) {
            Ok(commit) => commit,
            Err(e) => {
                eprintln!("Error parsing commit message: {}", e);
                eprintln!();
                eprintln!("Tip: Use interactive mode to create a valid commit message");
                return ExitCode::FAILURE;
            }
        };

        // Step 3: Preview
        println!("=== Preview ===");
        println!();
        println!("{}", commit_message.to_conventional_commit());
        println!();

        // Step 4: Execute commit
        println!("Executing git commit...");
        match self
            .commit_executor
            .execute(&commit_message.to_conventional_commit())
        {
            Ok(result) => {
                println!("✓ Commit created successfully!");
                println!("  SHA: {}", result.sha);
                println!("  Summary: {}", result.summary);
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("✗ Failed to create commit: {}", e);
                ExitCode::FAILURE
            }
        }
    }

    /// Parse a direct message string into a CommitMessage
    ///
    /// Supports two formats:
    /// 1. Simple format: "type(scope): description"
    /// 2. Full conventional commit format with body and footers
    fn parse_direct_message(&self, message: &str) -> Result<CommitMessage, String> {
        let message = message.trim();

        if message.is_empty() {
            return Err("Commit message cannot be empty".to_string());
        }

        // Split into header and rest
        let parts: Vec<&str> = message.splitn(2, '\n').collect();
        let header = parts[0];
        let rest = parts.get(1).map(|s| s.trim());

        // Parse header: type(scope)!: description
        let (commit_type, scope, breaking_in_header, description) = self.parse_header(header)?;

        // Parse body and footers from rest
        let (body, breaking_change) = if let Some(rest_text) = rest {
            self.parse_body_and_footers(rest_text, breaking_in_header)?
        } else {
            (None, None)
        };

        // Validate breaking change consistency
        if breaking_in_header && breaking_change.is_none() {
            return Err(
                "Breaking change indicator '!' in header requires BREAKING CHANGE footer"
                    .to_string(),
            );
        }

        CommitMessage::new(
            commit_type,
            scope,
            description.to_string(),
            body,
            breaking_change,
        )
        .map_err(|e| e.to_string())
    }

    /// Parse the commit message header
    /// Returns: (type, scope, has_breaking_indicator, description)
    fn parse_header(
        &self,
        header: &str,
    ) -> Result<(CommitType, Option<String>, bool, String), String> {
        // Find the colon that separates type/scope from description
        let colon_pos = header
            .find(':')
            .ok_or_else(|| "Invalid format: missing ':' separator".to_string())?;

        let type_scope_part = &header[..colon_pos];
        let description = header[colon_pos + 1..].trim();

        if description.is_empty() {
            return Err("Description cannot be empty".to_string());
        }

        // Check for breaking change indicator
        let (type_scope_part, has_breaking) = if type_scope_part.ends_with('!') {
            (&type_scope_part[..type_scope_part.len() - 1], true)
        } else {
            (type_scope_part, false)
        };

        // Parse type and scope
        let (commit_type, scope) = if let Some(paren_start) = type_scope_part.find('(') {
            // Has scope
            let commit_type_str = &type_scope_part[..paren_start];
            let scope_part = &type_scope_part[paren_start + 1..];

            let paren_end = scope_part
                .find(')')
                .ok_or_else(|| "Invalid format: unclosed scope parenthesis".to_string())?;

            let scope_str = &scope_part[..paren_end];

            if scope_part.len() > paren_end + 1 {
                return Err("Invalid format: unexpected characters after scope".to_string());
            }

            let commit_type = CommitType::from_str(commit_type_str).map_err(|e| e.to_string())?;

            (commit_type, Some(scope_str.to_string()))
        } else {
            // No scope
            let commit_type = CommitType::from_str(type_scope_part).map_err(|e| e.to_string())?;
            (commit_type, None)
        };

        Ok((commit_type, scope, has_breaking, description.to_string()))
    }

    /// Parse body and footers from the rest of the commit message
    /// Returns: (body, breaking_change)
    fn parse_body_and_footers(
        &self,
        text: &str,
        has_breaking_indicator: bool,
    ) -> Result<(Option<String>, Option<String>), String> {
        let text = text.trim();

        if text.is_empty() {
            return Ok((None, None));
        }

        // Look for footer section (starts with a line matching "KEY: value" or "KEY #value")
        let lines: Vec<&str> = text.lines().collect();
        let mut footer_start_idx = None;

        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            // Footer line pattern: word characters, optional spaces, colon or #, space, content
            if trimmed.contains(':') || trimmed.contains('#') {
                let parts: Vec<&str> = if trimmed.contains(':') {
                    trimmed.splitn(2, ':').collect()
                } else {
                    trimmed.splitn(2, '#').collect()
                };

                if parts.len() == 2 {
                    let key = parts[0].trim();
                    // Check if key looks like a footer key (no spaces, alphanumeric or hyphens)
                    if !key.is_empty()
                        && key
                            .chars()
                            .all(|c| c.is_alphanumeric() || c == '-' || c == ' ')
                        && key.to_uppercase() == key
                    {
                        footer_start_idx = Some(idx);
                        break;
                    }
                }
            }
        }

        let (body_text, footer_text) = if let Some(idx) = footer_start_idx {
            let body = if idx > 0 {
                Some(lines[..idx].join("\n").trim().to_string())
            } else {
                None
            };
            let footers = lines[idx..].join("\n");
            (body, Some(footers))
        } else {
            // No footers found, everything is body
            (Some(text.to_string()), None)
        };

        // Parse breaking change from footers
        let breaking_change = if let Some(footer_text) = footer_text {
            self.extract_breaking_change(&footer_text)?
        } else {
            None
        };

        // Validate that if we have a breaking indicator, we must have breaking change
        if has_breaking_indicator && breaking_change.is_none() {
            return Err(
                "Breaking change indicator '!' requires BREAKING CHANGE footer".to_string(),
            );
        }

        Ok((body_text, breaking_change))
    }

    /// Extract BREAKING CHANGE footer if present
    fn extract_breaking_change(&self, footer_text: &str) -> Result<Option<String>, String> {
        for line in footer_text.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("BREAKING CHANGE:") || trimmed.starts_with("BREAKING-CHANGE:") {
                let value = if let Some(colon_idx) = trimmed.find(':') {
                    trimmed[colon_idx + 1..].trim()
                } else {
                    continue;
                };

                if value.is_empty() {
                    return Err("BREAKING CHANGE footer cannot be empty".to_string());
                }

                return Ok(Some(value.to_string()));
            }
        }
        Ok(None)
    }

    /// Run in interactive mode
    fn run_interactive(&self) -> ExitCode {
        println!("CLI controller initialized");
        println!();

        // Step 1: Check for staged changes
        println!("Checking for staged changes...");
        match self.staging_checker.has_staged_changes() {
            Ok(true) => {
                println!("✓ Staged changes detected");
                println!();
            }
            Ok(false) => {
                println!("✗ No staged changes found");
                println!();
                println!("Please stage your changes first:");
                println!("  git add <files>");
                return ExitCode::FAILURE;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                return ExitCode::FAILURE;
            }
        }

        // Step 2: Collect initial commit message
        let commit = match self.input_collector.collect() {
            Ok(commit) => commit,
            Err(e) => {
                eprintln!("Error collecting commit message: {}", e);
                return ExitCode::FAILURE;
            }
        };

        // Convert to CommitData for editing
        let mut data = CommitData {
            commit_type: commit.commit_type(),
            scope: commit.scope().map(|s| s.to_string()),
            description: commit.description().to_string(),
            body: commit.body().map(|s| s.to_string()),
            breaking_change: commit.breaking_change().map(|s| s.to_string()),
        };

        // Step 3: Preview, confirm, and optionally edit loop
        loop {
            // Create commit message from data for preview AND later use
            let commit_message = match data.to_commit_message() {
                Ok(commit) => {
                    println!("=== Preview ===");
                    println!();
                    println!("{}", commit.to_conventional_commit());
                    println!();
                    commit
                }
                Err(e) => {
                    eprintln!("Error creating commit message: {}", e);
                    return ExitCode::FAILURE;
                }
            };

            // Ask for action
            match self.prompt_action() {
                Ok(Action::Proceed) => {
                    println!();
                    println!("Executing git commit...");

                    // ACTUALLY EXECUTE THE COMMIT
                    match self
                        .commit_executor
                        .execute(&commit_message.to_conventional_commit())
                    {
                        Ok(result) => {
                            println!("✓ Commit created successfully!");
                            println!("  SHA: {}", result.sha);
                            println!("  Summary: {}", result.summary);
                            return ExitCode::SUCCESS;
                        }
                        Err(e) => {
                            eprintln!("✗ Failed to create commit: {}", e);

                            // Offer to try dry-run to see what's wrong
                            println!(
                                "Would you like to try a dry-run to diagnose the issue? (y/N)"
                            );
                            print!("Choice: ");
                            io::stdout().flush().ok();

                            let mut input = String::new();
                            io::stdin().read_line(&mut input).ok();

                            if input.trim().to_lowercase() == "y" {
                                match self
                                    .commit_executor
                                    .dry_run(&commit_message.to_conventional_commit())
                                {
                                    Ok(_) => println!(
                                        "Dry-run succeeded. Please check your Git configuration."
                                    ),
                                    Err(dry_run_err) => {
                                        println!("Dry-run also failed: {}", dry_run_err)
                                    }
                                }
                            }

                            return ExitCode::FAILURE;
                        }
                    }
                }
                Ok(Action::Edit) => {
                    match self.edit_commit(&mut data) {
                        Ok(()) => continue, // Show preview again
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            return ExitCode::FAILURE;
                        }
                    }
                }
                Ok(Action::Abort) => {
                    println!();
                    println!("Commit aborted.");
                    return ExitCode::FAILURE;
                }
                Err(_) => {
                    return ExitCode::FAILURE;
                }
            }
        }
    }

    fn prompt_action(&self) -> Result<Action, ExitCode> {
        println!("What would you like to do?");
        println!("  y - Proceed with commit");
        println!("  e - Edit a field");
        println!("  n - Abort");
        println!();
        print!("Choice (y/e/n): ");
        io::stdout().flush().map_err(|_| ExitCode::FAILURE)?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|_| ExitCode::FAILURE)?;
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "y" | "yes" | "proceed" => Ok(Action::Proceed),
            "e" | "edit" => Ok(Action::Edit),
            "n" | "no" | "abort" | "cancel" => Ok(Action::Abort),
            "" => Ok(Action::Proceed), // Default to proceed on Enter
            _ => {
                println!("Invalid choice. Please enter y, e, or n.");
                println!();
                self.prompt_action()
            }
        }
    }

    fn edit_commit(&self, data: &mut CommitData) -> Result<(), String> {
        println!();
        println!("Which field would you like to edit?");
        println!(
            "  1 - Type       (currently: {})",
            data.commit_type.as_str()
        );
        println!(
            "  2 - Scope      (currently: {})",
            data.scope.as_deref().unwrap_or("<none>")
        );
        println!("  3 - Description");
        println!(
            "  4 - Body       (currently: {})",
            if data.body.is_some() {
                "<set>"
            } else {
                "<none>"
            }
        );
        println!(
            "  5 - Breaking   (currently: {})",
            if data.breaking_change.is_some() {
                "<set>"
            } else {
                "<none>"
            }
        );
        println!();
        print!("Field (1-5): ");
        io::stdout().flush().map_err(|e| e.to_string())?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| e.to_string())?;
        let input = input.trim();

        let new_data = self.input_collector.edit_field(data, input)?;
        *data = new_data;

        Ok(())
    }
}

impl Default for CliController {
    fn default() -> Self {
        Self::new()
    }
}

enum Action {
    Proceed,
    Edit,
    Abort,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_controller_can_be_created() {
        let _controller = CliController::new();
    }

    #[test]
    fn cli_controller_has_default() {
        let _controller = CliController::default();
    }

    #[test]
    fn cli_controller_can_be_created_with_mode() {
        let _interactive = CliController::with_mode(CliMode::Interactive);
        let _direct = CliController::with_mode(CliMode::Direct {
            message: "feat: add new feature".to_string(),
        });
    }

    #[test]
    fn parse_simple_commit_message() {
        let controller = CliController::new();
        let result = controller.parse_direct_message("feat: add new feature");
        assert!(result.is_ok());
    }

    #[test]
    fn parse_commit_message_with_scope() {
        let controller = CliController::new();
        let result = controller.parse_direct_message("feat(api): add new endpoint");
        assert!(result.is_ok());
    }

    #[test]
    fn parse_commit_message_with_breaking_change() {
        let controller = CliController::new();
        let message = "feat(api)!: change API structure\n\nBREAKING CHANGE: API now uses v2 format";
        let result = controller.parse_direct_message(message);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_commit_message_with_body() {
        let controller = CliController::new();
        let message = "feat: add new feature\n\nThis is a detailed body explaining the change.";
        let result = controller.parse_direct_message(message);
        assert!(result.is_ok());
    }

    #[test]
    fn reject_empty_message() {
        let controller = CliController::new();
        let result = controller.parse_direct_message("");
        assert!(result.is_err());
    }

    #[test]
    fn reject_message_without_colon() {
        let controller = CliController::new();
        let result = controller.parse_direct_message("feat add new feature");
        assert!(result.is_err());
    }

    #[test]
    fn reject_breaking_indicator_without_footer() {
        let controller = CliController::new();
        let result = controller.parse_direct_message("feat!: breaking change without footer");
        assert!(result.is_err());
    }
}
