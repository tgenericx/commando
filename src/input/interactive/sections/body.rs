/// Body section — optional multiline commit body.
///
/// Asks first before launching into multiline collection.
/// Blank initial response skips the section entirely.
use crate::input::interactive::InteractiveError;
use crate::ports::ui::Ui;

pub fn collect<U: Ui>(ui: &U) -> Result<Option<String>, InteractiveError> {
    let options = vec![
        (
            "yes".to_string(),
            "Yes".to_string(),
            "Add a body with more detail".to_string(),
        ),
        (
            "no".to_string(),
            "No".to_string(),
            "Skip the body".to_string(),
        ),
    ];

    let choice = ui
        .select("4. Add a body with more detail?", options)
        .map_err(InteractiveError::Ui)?;

    if choice == "no" {
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
