use std::io::{self, Read};

use crate::input::InputError;

pub struct PasteReader;

impl PasteReader {
    pub fn new() -> Self {
        Self
    }

    pub fn read(&self) -> Result<String, InputError> {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer.trim().to_string())
    }
}

impl Default for PasteReader {
    fn default() -> Self {
        Self::new()
    }
}
