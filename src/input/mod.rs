use crate::compiler::compile::compile;

mod error;
mod interactive;

pub use error::InputError;

#[derive(Debug)]
pub enum InputMode {
    Interactive,
    Validate(String),
    Edit(String),
}

#[derive(Debug, Default)]
pub struct InputCollector;

impl InputCollector {
    pub fn new() -> Self {
        Self
    }

    pub fn collect(&self, mode: InputMode) -> Result<String, InputError> {
        match mode {
            InputMode::Interactive => {
                let mut interactive = interactive::InteractiveMode::new();
                interactive.collect()
            }
            InputMode::Validate(msg) => {
                compile(&msg)?;
                Ok(msg)
            }
            InputMode::Edit(original) => {
                // Show errors and launch interactive
                if let Err(e) = compile(&original) {
                    println!("Issues found:\n  {}\n", e);
                }
                println!("Let's fix this together.\n");

                let mut interactive = interactive::InteractiveMode::new();
                interactive.collect()
            }
        }
    }
}
