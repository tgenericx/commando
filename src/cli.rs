use std::process::ExitCode;

#[derive(Default)]
pub struct CliController;

impl CliController {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self) -> ExitCode {
        println!("CLI controller initialized");
        println!("Staging check: not implemented");
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
    fn run_returns_success_exit_code() {
        let controller = CliController::new();
        let exit_code = controller.run();

        assert_eq!(exit_code, ExitCode::SUCCESS);
    }
}
