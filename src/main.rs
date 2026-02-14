mod adapters;
mod cli;
mod domain;
mod input_collector;
mod ports;

use cli::CliController;

use crate::adapters::{GitCommitExecutor, GitStagingChecker};

fn main() -> std::process::ExitCode {
    // Create the concrete implementations
    let staging_checker = GitStagingChecker;
    let commit_executor = GitCommitExecutor;

    // Inject them into the controller
    let controller = CliController::new(staging_checker, commit_executor);

    // Run the application
    controller.run()
}
