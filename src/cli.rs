use std::io::{self, Write};
use std::process::ExitCode;

use crate::input_collector::{CommitData, InputCollector};
use crate::staging_checker::StagingChecker;

pub struct CliController {
    staging_checker: StagingChecker,
    input_collector: InputCollector,
}

impl CliController {
    pub fn new() -> Self {
        Self {
            staging_checker: StagingChecker::new(),
            input_collector: InputCollector::new(),
        }
    }

    pub fn run(&self) -> ExitCode {
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
            // Show preview
            match data.to_commit_message() {
                Ok(commit) => {
                    println!("=== Preview ===");
                    println!();
                    println!("{}", commit.to_conventional_commit());
                    println!();
                }
                Err(e) => {
                    eprintln!("Error creating commit message: {}", e);
                    return ExitCode::FAILURE;
                }
            }

            // Ask for action
            match self.prompt_action() {
                Ok(Action::Proceed) => {
                    println!();
                    println!("✓ Commit message created successfully!");
                    println!("(Git commit execution not yet implemented)");
                    return ExitCode::SUCCESS;
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
}
