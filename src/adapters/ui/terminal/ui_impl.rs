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

    fn select<T: Clone>(
        &self,
        title: &str,
        options: Vec<(T, String, String)>,
    ) -> Result<T, UiError> {
        if options.is_empty() {
            return Err(UiError("No options provided to select".into()));
        }

        println!();
        println!("{title}");
        println!("{}", "-".repeat(title.len()));

        for (i, (_, label, description)) in options.iter().enumerate() {
            println!("{}. {} - {}", i + 1, label, description);
        }

        loop {
            print!("\nEnter choice number: ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            if let Ok(index) = input.trim().parse::<usize>() {
                if index >= 1 && index <= options.len() {
                    return Ok(options[index - 1].0.clone());
                }
            }

            println!(
                "Invalid selection. Please enter a number between 1 and {}.",
                options.len()
            );
        }
    }

    fn show_preview(&self, content: &str) {
        println!("\n=== Preview ===\n");
        println!("{}", content);
        println!();
    }

    fn println(&self, msg: &str) {
        println!("{}", msg);
    }
}
