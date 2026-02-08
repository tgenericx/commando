mod cli;
mod commit_message;
mod staging_checker;
use cli::CliController;
use std::process::ExitCode;

fn main() -> ExitCode {
    let controller = CliController::new();

    controller.run()
}
