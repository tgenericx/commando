use std:: fs;
use std:: path:: Path;

use crate:: input:: InputError;

pub struct FileReader;

impl FileReader {
    pub fn new () -> Self {
    Self
  }

    pub fn read(& self, path: & str) -> Result < String, InputError > {
    let content = fs:: read_to_string(Path::new(path))?;
  Ok(content.trim().to_string())
}
}

impl Default for FileReader {
  fn default() -> Self {
    Self:: new()
  }
}
