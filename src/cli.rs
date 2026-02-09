use crate::input_collector::InputCollector;
use crate::staging_checker::StagingChecker;
use std::process::ExitCode;

#[derive(Default)]
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

        // Step 2: Collect commit message from user
        let commit = match self.input_collector.collect() {
            Ok(commit) => commit,
            Err(e) => {
                eprintln!("Error collecting commit message: {}", e);
                return ExitCode::FAILURE;
            }
        };

        // Step 3: Show preview
        println!("=== Preview ===");
        println!();
        println!("{}", commit.to_conventional_commit());
        println!();

        // Step 4: Confirm (for now, just show message)
        println!("Commit message created successfully!");
        println!("(Git commit execution not yet implemented)");

        ExitCode::SUCCESS
    }
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
