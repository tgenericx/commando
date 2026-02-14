/// Body section — optional multiline commit body.
///
/// Asks first before launching into multiline collection.
/// Blank initial response skips the section entirely.
use crate::domain::DomainError;
use crate::ports::ui::Ui;
use std::io;

pub fn collect<U: Ui>(ui: &U) -> Result<Option<String>, DomainError> {
    let wants_body = ui
        .confirm("4. Add a body with more detail?")
        .map_err(|_e| DomainError::EmptyBody)?;

    if !wants_body {
        ui.println("");
        return Ok(None);
    }

    ui.println("");
    ui.println("Enter body (blank line or Ctrl+D to finish):");
    ui.println("");

    let mut lines: Vec<String> = Vec::new();

    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break, // Ctrl+D / EOF
            Ok(_) => {
                let trimmed = line.trim_end_matches('\n').trim_end_matches('\r');
                if trimmed.is_empty() && !lines.is_empty() {
                    break; // blank line ends input
                }
                lines.push(trimmed.to_string());
            }
            Err(_e) => return Err(DomainError::EmptyBody), // io error — treat as skip
        }
    }

    let body = lines.join("\n").trim().to_string();
    ui.println("");

    if body.is_empty() {
        Ok(None)
    } else {
        Ok(Some(body))
    }
}
