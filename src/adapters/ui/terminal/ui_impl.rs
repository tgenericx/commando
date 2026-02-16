/// Simple terminal UI — basic implementation without fancy TUI features.
///
/// This is a fallback implementation that works everywhere but doesn't
/// have the rich interactive features of RatatuiUI.
use crate::ports::ui::{Ui, UiError};
use std::io::{self, Write};

#[derive(Copy, Clone)]
pub struct TerminalUI;

impl Ui for TerminalUI {
    fn prompt(&self, label: &str) -> Result<String, UiError> {
        print!("{}", label);
        io::stdout().flush().map_err(|e| UiError(e.to_string()))?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| UiError(e.to_string()))?;

        Ok(input.trim().to_string())
    }

    fn show_preview(&self, content: &str) {
        println!("\n=== Preview ===\n");
        println!("{}", content);
        println!();
    }

    fn confirm(&self, msg: &str) -> Result<bool, UiError> {
        print!("{} (y/n): ", msg);
        io::stdout().flush().map_err(|e| UiError(e.to_string()))?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| UiError(e.to_string()))?;

        let response = input.trim().to_lowercase();
        Ok(response == "y" || response == "yes")
    }

    fn select(
        &self,
        title: &str,
        options: Vec<(String, String, String)>,
    ) -> Result<String, UiError> {
        println!("{}", title);
        println!();

        // Display all options with numbers
        for (idx, (_, label, description)) in options.iter().enumerate() {
            println!("  {}. {:<12} — {}", idx + 1, label, description);
        }
        println!();

        loop {
            print!("Select (1-{}): ", options.len());
            io::stdout().flush().map_err(|e| UiError(e.to_string()))?;

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| UiError(e.to_string()))?;

            let input = input.trim();

            // Try to parse as number
            if let Ok(num) = input.parse::<usize>() {
                if num >= 1 && num <= options.len() {
                    return Ok(options[num - 1].0.clone());
                }
            }

            // Try to match by value or label
            for (value, label, _) in &options {
                if input.eq_ignore_ascii_case(value) || input.eq_ignore_ascii_case(label) {
                    return Ok(value.clone());
                }
            }

            println!(
                "  ✗ Invalid selection. Please enter a number from 1 to {}",
                options.len()
            );
        }
    }

    fn println(&self, msg: &str) {
        println!("{}", msg);
    }
}
