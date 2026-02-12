use crate::compiler::compile::compile;
use crate::input::InputError;

mod builder;
mod prompt;
mod validator;

pub use builder::MessageBuilder;
pub use prompt::Prompt;

#[derive(Default)]
pub struct InteractiveMode {
    prompt: Prompt,
    builder: MessageBuilder,
}

impl InteractiveMode {
    pub fn new() -> Self {
        Self {
            prompt: Prompt::new(),
            builder: MessageBuilder::new(),
        }
    }

    pub fn collect(&mut self) -> Result<String, InputError> {
        println!("\n=== Interactive Commit Builder ===\n");

        loop {
            if self.builder.commit_type().is_none() {
                let ty = self.prompt.commit_type()?;
                self.builder.with_type(ty);
                continue;
            }

            if self.builder.scope().is_none() {
                let scope = self.prompt.scope()?;
                self.builder.with_scope(scope);
                continue;
            }

            if self.builder.breaking().is_none() {
                let breaking = self.prompt.breaking()?;
                self.builder.with_breaking(breaking);
                continue;
            }

            if self.builder.description().is_none() {
                let description = self.prompt.description()?;
                self.builder.with_description(description);
                continue;
            }

            if self.builder.body().is_none() {
                let body = self.prompt.body()?;
                self.builder.with_body(body);
                continue;
            }

            if self.builder.breaking() == Some(true)
                && self.builder.breaking_description().is_none()
            {
                let breaking_desc = self.prompt.breaking_description()?;
                self.builder.with_breaking_description(breaking_desc);
                continue;
            }

            if self.builder.footers().is_none() {
                let footers = self.prompt.footers()?;
                self.builder.with_footers(footers);
                continue;
            }

            let message = self.builder.build()?;

            println!("\n{}", "=".repeat(50));
            println!("CURRENT MESSAGE:");
            println!("{}", message);
            println!("{}", "=".repeat(50));
            println!();

            println!("Options:");
            println!("  [c] Continue and commit");
            println!("  [e] Edit message");
            println!("  [r] Restart from beginning");
            println!("  [q] Cancel");

            let choice = self.prompt.ask("Choice")?;

            match choice.to_lowercase().as_str() {
                "c" | "continue" => {
                    let formatted = compile(&message)?;
                    return Ok(formatted);
                }
                "e" | "edit" => {
                    self.prompt_edit_field()?;
                }
                "r" | "restart" => {
                    self.builder = MessageBuilder::new();
                    println!("\nRestarting...\n");
                }
                "q" | "quit" | "cancel" => {
                    return Err(InputError::Cancelled);
                }
                _ => {
                    println!("  ✗ Invalid choice\n");
                }
            }
        }
    }

    fn prompt_edit_field(&mut self) -> Result<(), InputError> {
        println!("\nWhich field do you want to edit?");
        println!(
            "  [1] Type (current: {})",
            self.builder
                .commit_type()
                .map(|s| s.as_str())
                .unwrap_or("none")
        );
        println!(
            "  [2] Scope (current: {})",
            self.builder
                .scope()
                .and_then(|opt| opt.as_ref())
                .map(|s| s.as_str())
                .unwrap_or("none")
        );
        println!(
            "  [3] Breaking change (current: {})",
            if self.builder.breaking() == Some(true) {
                "yes"
            } else if self.builder.breaking() == Some(false) {
                "no"
            } else {
                "not set"
            }
        );
        println!(
            "  [4] Description (current: {})",
            self.builder
                .description()
                .map(|s| s.as_str())
                .unwrap_or("none")
        );
        println!("  [5] Body");
        println!("  [6] Breaking description");
        println!("  [7] Footers");
        println!("  [b] Back");

        let choice = self.prompt.ask("Field")?;

        match choice.as_str() {
            "1" => {
                let ty = self.prompt.commit_type()?;
                self.builder.with_type(ty);
            }
            "2" => {
                let scope = self.prompt.scope()?;
                self.builder.with_scope(scope);
            }
            "3" => {
                let breaking = self.prompt.breaking()?;
                self.builder.with_breaking(breaking);
            }
            "4" => {
                let description = self.prompt.description()?;
                self.builder.with_description(description);
            }
            "5" => {
                let body = self.prompt.body()?;
                self.builder.with_body(body);
            }
            "6" => {
                if self.builder.breaking() == Some(true) {
                    let breaking_desc = self.prompt.breaking_description()?;
                    self.builder.with_breaking_description(breaking_desc);
                } else {
                    println!("  ✗ Not a breaking change");
                }
            }
            "7" => {
                let footers = self.prompt.footers()?;
                self.builder.with_footers(footers);
            }
            "b" | "back" => {}
            _ => println!("  ✗ Invalid choice"),
        }

        Ok(())
    }
}
