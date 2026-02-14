/// Body section — optional multiline commit body.
///
/// Asks first before launching into multiline collection.
/// Blank initial response skips the section entirely.
use crate::domain::DomainError;
use crate::input::interactive::InteractiveError;
use crate::ports::ui::Ui;

pub fn collect<U: Ui>(ui: &U) -> Result<Option<String>, InteractiveError> {
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
        let input = ui.prompt("")?;
        if input.is_empty() && !lines.is_empty() {
            break;
        }
        lines.push(input);
    }

    let body = lines.join("\n").trim().to_string();
    ui.println("");

    if body.is_empty() {
        Ok(None)
    } else {
        Ok(Some(body))
    }
}
