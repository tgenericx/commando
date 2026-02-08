pub struct CliController;

impl CliController {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self) -> i32 {
        println!("CLI controller initialized");
        println!("Staging check: not implemented");
        0
    }
}

impl Default for CliController {
    fn default() -> Self {
        Self::new()
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

        assert_eq!(exit_code, 0);
    }
}
