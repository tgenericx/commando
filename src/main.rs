mod cli;
use cli::CliController;

fn main() {
    let controller = CliController::new();

    controller.run();
}
