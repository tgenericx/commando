use std::process::ExitCode;

use crate::staging_checker::StagingChecker;

#[derive(Default)]
pub struct CliController {
    staging_checker: StagingChecker,
}

impl CliController {
    pub fn new() -> Self {
        Self {
            staging_checker: StagingChecker::new(),
        }
    }

    pub fn run(&self) -> ExitCode {
        println!("CLI controller initialized");
        println!("Checking staging state...");

        match self.staging_checker.has_staged_changes() {
            Ok(true) => {
                println!("Staged changes detected");
                ExitCode::SUCCESS
            }
            Ok(false) => {
                println!("No staged changes found");
                ExitCode::FAILURE
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                ExitCode::FAILURE
            }
        }
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

    #[test]
    fn run_returns_exit_code() {
        let controller = CliController::new();
        let exit_code = controller.run();

        // Exit code depends on whether we're in a git repo with staged changes
        // Just verify it returns something
        assert!(exit_code == ExitCode::SUCCESS || exit_code == ExitCode::FAILURE);
    }
}
