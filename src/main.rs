mod cli;
mod commit_types;
mod compiler;
mod editor;
mod git;
mod message;
mod staging_checker;
mod validation;

use cli::CliController;
use std::process::ExitCode;

fn main() -> ExitCode {
    let controller = CliController::new();

    controller.run()
}
