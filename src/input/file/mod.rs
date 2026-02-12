use crate::compiler::compile::compile;
use crate::input::InputError;

mod reader;

pub use reader::FileReader;

pub struct FileMode {
    reader: FileReader,
}

impl FileMode {
    pub fn new() -> Self {
        Self {
            reader: FileReader::new(),
        }
    }

    pub fn collect(&self, path: &str) -> Result<String, InputError> {
        println!("\nReading commit message from: {}\n", path);

        let content = self.reader.read(path)?;
        
        if content.is_empty() {
            return Err(InputError::Empty);
        }

        println!("Validating and formatting...");
        let formatted = compile(&content)?;
        println!("✓ Valid commit message!\n");
        
        Ok(formatted)
    }
}

impl Default for FileMode {
    fn default() -> Self {
        Self::new()
    }
}
