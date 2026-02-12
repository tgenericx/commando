mod error;
mod interactive;

pub use error::InputError;

#[derive(Debug)]
pub enum InputMode {
    Interactive,
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
        }
    }
}
