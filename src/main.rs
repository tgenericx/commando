mod cli;
mod commit_executor;
mod domain;
mod input_collector;
mod ports;
mod staging_checker;

use cli::CliController;
use std::process::ExitCode;

fn main() -> ExitCode {
    let controller = CliController::new();
    controller.run()
}
