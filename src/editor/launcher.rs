use std::path::Path;
use std::process::{Command, Stdio};

use crate::cli::CliError;

pub fn launch(editor: &str, path: &Path) -> Result<(), CliError> {
    let path_str = path
        .to_str()
        .ok_or_else(|| CliError::Editor("Invalid file path".to_string()))?;

    let status = if cfg!(windows) {
        Command::new("cmd")
            .args(["/c", editor, path_str])
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
    } else {
        Command::new(editor)
            .arg(path_str)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
    };

    match status {
        Ok(status) if status.success() => Ok(()),
        Ok(_) => Err(CliError::Editor(
            "Editor exited with non-zero status".to_string(),
        )),
        Err(e) => Err(CliError::Editor(format!("Failed to launch editor: {}", e))),
    }
}
