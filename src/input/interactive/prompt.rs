use std::io::{self, BufRead, Write};

use super::validator::InteractiveValidator;
use crate::input::InputError;

pub struct Prompt {
    validator: InteractiveValidator,
}

impl Prompt {
    pub fn new() -> Self {
        Self {
            validator: InteractiveValidator::new(),
        }
    }

    pub fn commit_type(&self) -> Result<String, InputError> {
        println!("Commit Type:");
        println!("  feat      - New feature");
        println!("  fix       - Bug fix");
        println!("  docs      - Documentation");
        println!("  style     - Formatting changes");
        println!("  refactor  - Code restructuring");
        println!("  perf      - Performance improvement");
        println!("  test      - Add/update tests");
        println!("  build     - Build system changes");
        println!("  ci        - CI configuration");
        println!("  chore     - Maintenance tasks");
        println!("  revert    - Revert previous commit");
        println!();

        loop {
            let input = self.ask("Type")?;

            if self.validator.validate_type(&input) {
                return Ok(input);
            }
            println!("  ✗ Invalid type. Choose from the list above.\n");
        }
    }

    pub fn scope(&self) -> Result<Option<String>, InputError> {
        println!("Scope (optional - press Enter to skip):");
        println!("  Examples: api, parser, auth, ui");
        println!("  Must be lowercase, alphanumeric with hyphens/underscores");
        println!();

        loop {
            let input = self.ask("Scope")?;

            if input.is_empty() {
                return Ok(None);
            }

            if self.validator.validate_scope(&input) {
                return Ok(Some(input));
            }
            println!("  ✗ Invalid scope. Must be lowercase alphanumeric.\n");
        }
    }

    pub fn breaking(&self) -> Result<bool, InputError> {
        println!("Is this a breaking change? (y/N)");
        let input = self.ask("Breaking")?;
        Ok(input == "y" || input == "yes")
    }

    pub fn description(&self) -> Result<String, InputError> {
        println!("\nDescription (max 72 characters):");
        println!("  Keep it concise and imperative (e.g., 'add' not 'added')");
        println!();

        loop {
            let input = self.ask("Description")?;

            if input.is_empty() {
                println!("  ✗ Description cannot be empty.\n");
                continue;
            }

            if !self.validator.validate_description(&input) {
                println!("  ✗ Too long ({} chars). Max 72.\n", input.len());
                continue;
            }

            return Ok(input);
        }
    }

    pub fn body(&self) -> Result<Option<String>, InputError> {
        println!("\nAdd detailed body? (y/N)");
        let input = self.ask("Choice")?;

        if input != "y" && input != "yes" {
            return Ok(None);
        }

        println!("\nEnter body (press Ctrl+D when done):");
        println!("  Explain what and why, not how");
        println!("  Text will be wrapped at 72 characters");
        println!();

        let body = self.read_multiline()?;
        Ok(if body.trim().is_empty() {
            None
        } else {
            Some(body)
        })
    }

    pub fn breaking_description(&self) -> Result<String, InputError> {
        println!("\nDescribe the breaking change:");
        println!("  Explain what breaks and how to migrate");
        println!();

        loop {
            let input = self.ask("Breaking change")?;

            if !input.is_empty() {
                return Ok(input);
            }
            println!("  ✗ Breaking change description cannot be empty.\n");
        }
    }

    pub fn footers(&self) -> Result<Option<String>, InputError> {
        println!("\nAdd footers (refs, closes, etc.)? (y/N)");
        let input = self.ask("Choice")?;

        if input != "y" && input != "yes" {
            return Ok(None);
        }

        println!("\nEnter footers (one per line, Ctrl+D when done):");
        println!("  Examples:");
        println!("    Refs: #123");
        println!("    Closes: #456");
        println!("    Co-authored-by: Name <email>");
        println!();

        let footers = self.read_multiline()?;
        Ok(if footers.trim().is_empty() {
            None
        } else {
            Some(footers)
        })
    }

    fn ask(&self, label: &str) -> Result<String, InputError> {
        print!("{}: ", label);
        io::stdout().flush().map_err(InputError::Io)?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(InputError::Io)?;
        Ok(input.trim().to_string())
    }

    fn read_multiline(&self) -> Result<String, InputError> {
        let mut content = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        loop {
            let mut line = String::new();
            match handle.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => content.push_str(&line),
                Err(e) => return Err(InputError::Io(e)),
            }
        }
        Ok(content.trim().to_string())
    }
}

impl Default for Prompt {
    fn default() -> Self {
        Self::new()
    }
}
