use crate::compiler::compile::compile;
use crate::input::InputError;

mod reader;

pub use reader::PasteReader;

pub struct PasteMode {
    reader: PasteReader,
}

impl PasteMode {
    pub fn new() -> Self {
        Self {
            reader: PasteReader::new(),
        }
    }

    pub fn collect(&self) -> Result<String, InputError> {
        println!("\n=== Paste Commit Message ===\n");
        println!("Paste your commit message below.");
        println!("Press Ctrl+D when done (Ctrl+Z on Windows):");
        println!();

        let input = self.reader.read()?;
        
        if input.is_empty() {
            return Err(InputError::Empty);
        }

        println!("\nValidating and formatting...");
        let formatted = compile(&input)?;
        println!("✓ Valid commit message!\n");
        
        Ok(formatted)
    }
}

impl Default for PasteMode {
    fn default() -> Self {
        Self::new()
    }
}
