mod cli;
mod commit_executor;
mod commit_message;
mod compiler;
mod input_collector;
mod staging_checker;

use cli::CliController;
use std::process::ExitCode;

fn main() -> ExitCode {
    let controller = CliController::new();
    controller.run()
}
