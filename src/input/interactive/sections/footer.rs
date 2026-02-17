use crate::input::interactive::InteractiveError;
/// Footer section — breaking change description, issue refs, co-authors.
///
/// Breaking change here is the source of truth. The '!' marker in the
/// header is derived automatically by CommitMessage when
/// breaking_change.is_some() — we never ask about it separately.
use crate::ports::ui::Ui;

pub fn collect_breaking_change<U: Ui>(ui: &U) -> Result<Option<String>, InteractiveError> {
    // Use select with yes/no options instead of confirm
    let options = vec![
        (
            "yes".to_string(),
            "Yes".to_string(),
            "This is a breaking change".to_string(),
        ),
        (
            "no".to_string(),
            "No".to_string(),
            "Not a breaking change".to_string(),
        ),
    ];

    let choice = ui
        .select("5. Does this break existing functionality?", options)
        .map_err(InteractiveError::Ui)?;

    if choice == "no" {
        ui.println("");
        return Ok(None);
    }

    ui.println("");
    ui.println("Describe what breaks and how users should adapt:");
    ui.println("");

    loop {
        let input = ui
            .prompt("Breaking change: ")
            .map_err(InteractiveError::Ui)?;

        if input.is_empty() {
            ui.println("  ✗ Description cannot be empty. Press Ctrl+C to abort.");
            continue;
        }

        ui.println("");
        return Ok(Some(input));
    }
}

pub fn collect_refs<U: Ui>(ui: &U) -> Result<Option<String>, InteractiveError> {
    ui.println("6. Issue references (optional — press Enter to skip):");
    ui.println("   e.g. #123, closes #456");
    ui.println("");

    let input = ui.prompt("Refs: ").map_err(InteractiveError::Ui)?;

    ui.println("");

    if input.is_empty() {
        Ok(None)
    } else {
        Ok(Some(input))
    }
}
