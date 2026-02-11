use std::path::Path;

use crate::cli::CliError;

mod launcher;
mod resolver;

#[derive(Default, Debug, Clone)]
pub struct Editor {
    command: String,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            command: resolver::resolve(),
        }
    }

    pub fn open(&self, path: &Path) -> Result<(), CliError> {
        launcher::launch(&self.command, path)
    }

    #[cfg(test)]
    pub fn with_command(command: &str) -> Self {
        Self {
            command: command.to_string(),
        }
    }
}
