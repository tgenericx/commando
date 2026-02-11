use std::fs;

use crate::cli::CliError;
use crate::compiler::compile;
use crate::editor::Editor;

mod file;
mod parser;
mod template;

#[derive(Debug, Default)]
pub struct MessageCollector;

impl MessageCollector {
    pub fn new() -> Self {
        Self
    }

    pub fn collect_from_editor(&self, editor: &Editor) -> Result<String, CliError> {
        let path = file::create_persistent_temp()?;

        let mut is_first_attempt = true;

        loop {
            if is_first_attempt {
                file::write_template(&path)?;
                is_first_attempt = false;
            }

            editor.open(&path)?;

            let content = fs::read_to_string(&path)?;
            let message = parser::strip_comments(&content);

            if message.is_empty() {
                eprintln!("\n✗ No commit message provided.");
                eprint!("This will abort the commit. Continue? (y/N): ");

                let mut choice = String::new();
                std::io::stdin().read_line(&mut choice).ok();

                if choice.trim().to_lowercase() == "y" {
                    let _ = file::cleanup(&path);
                    return Err(CliError::EmptyMessage);
                }

                // User wants to try again
                eprintln!("Reopening editor...\n");
                continue;
            }

            match compile::compile(&message) {
                Ok(formatted) => {
                    let _ = file::cleanup(&path);
                    return Ok(formatted);
                }
                Err(err) => {
                    eprintln!("\n✗ Compilation failed:");
                    eprintln!("{}", err);

                    eprint!("\nReopen editor to fix? (Y/n): ");
                    let mut choice = String::new();
                    std::io::stdin().read_line(&mut choice).ok();

                    if choice.trim().to_lowercase() == "n" {
                        let _ = file::cleanup(&path);
                        return Err(CliError::CompileFailed);
                    }

                    // Append compiler error as comment
                    let annotated = format!(
                        "{}\n\n# {}\n# {}\n#\n# Fix the error above and save again.\n",
                        content,
                        "=".repeat(50),
                        err
                    );
                    file::write_content(&path, &annotated)?;
                }
            }
        }
    }
}
