mod cli;
mod staging_checker;
use cli::CliController;

fn main() {
    let controller = CliController::new();

    controller.run();
}
