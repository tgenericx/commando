/// Terminal UI adapter — plain stdin/stdout implementation of the Ui port.
///
/// This is the production UI. RatatuiUI will be a second impl of the same
/// trait. Swapping them requires changing one line in cli.rs.
use std::io::{self, Write};

use crate::ports::ui::{Ui, UiError};

pub struct TerminalUI;

impl Ui for TerminalUI {
    fn prompt(&self, label: &str) -> Result<String, UiError> {
        print!("{}", label);
        io::stdout().flush().map_err(UiError::from)?;
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).map_err(UiError::from)?;
        Ok(buf.trim().to_string())
    }

    fn show_preview(&self, content: &str) {
        println!();
        println!("=== Preview ===");
        println!();
        println!("{}", content);
        println!();
    }

    fn confirm(&self, msg: &str) -> Result<bool, UiError> {
        let input = self.prompt(&format!("{} (y/N): ", msg))?;
        Ok(matches!(input.to_lowercase().as_str(), "y" | "yes"))
    }

    fn println(&self, msg: &str) {
        println!("{}", msg);
    }
}
