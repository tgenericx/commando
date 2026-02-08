mod cli;
use cli::CliController;

fn main() {
    let controller = CliController::new();
    let exit_code = controller.run();

    std::process::exit(exit_code);
}
