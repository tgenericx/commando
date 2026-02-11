use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn create_persistent_temp() -> io::Result<PathBuf> {
    let mut path = std::env::temp_dir();
    let filename = format!("commando-msg-{}.txt", std::process::id());
    path.push(filename);

    // Create empty file
    fs::write(&path, "")?;
    Ok(path)
}

pub fn write_content(path: &Path, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

pub fn cleanup(path: &Path) -> io::Result<()> {
    fs::remove_file(path)
}

pub fn write_template(path: &Path) -> io::Result<()> {
    fs::write(path, super::template::TEMPLATE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_persistent_temp_file() {
        let path = create_persistent_temp().unwrap();
        assert!(path.exists());
        cleanup(&path).unwrap();
    }

    #[test]
    fn writes_content_to_file() {
        let path = create_persistent_temp().unwrap();
        write_content(&path, "test content").unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "test content");
        cleanup(&path).unwrap();
    }
}
