/// Header section — commit type, scope, description.
///
/// Each function validates its field immediately at prompt time.
/// A bad value is rejected before the user moves on — no post-hoc
/// validation needed for these fields.
use crate::domain::{CommitMessage, CommitType};
use crate::input::interactive::InteractiveError;
use crate::ports::ui::Ui;

/// Collect commit type using a selection UI for better UX
pub fn collect_type<U: Ui>(ui: &U) -> Result<CommitType, InteractiveError> {
    let options = vec![
        (
            "feat".to_string(),
            "feat".to_string(),
            "new feature".to_string(),
        ),
        ("fix".to_string(), "fix".to_string(), "bug fix".to_string()),
        (
            "docs".to_string(),
            "docs".to_string(),
            "documentation only".to_string(),
        ),
        (
            "style".to_string(),
            "style".to_string(),
            "formatting, whitespace".to_string(),
        ),
        (
            "refactor".to_string(),
            "refactor".to_string(),
            "code restructuring".to_string(),
        ),
        (
            "perf".to_string(),
            "perf".to_string(),
            "performance improvement".to_string(),
        ),
        (
            "test".to_string(),
            "test".to_string(),
            "adding or fixing tests".to_string(),
        ),
        (
            "build".to_string(),
            "build".to_string(),
            "build system / dependencies".to_string(),
        ),
        (
            "ci".to_string(),
            "ci".to_string(),
            "CI configuration".to_string(),
        ),
        (
            "chore".to_string(),
            "chore".to_string(),
            "maintenance".to_string(),
        ),
        (
            "revert".to_string(),
            "revert".to_string(),
            "revert a previous commit".to_string(),
        ),
    ];

    let selected = ui
        .select("1. Select commit type:", options)
        .map_err(InteractiveError::Ui)?;

    CommitType::from_str(&selected).map_err(|_| {
        InteractiveError::Ui(crate::ports::ui::UiError(
            "Invalid commit type selected".to_string(),
        ))
    })
}

pub fn collect_scope<U: Ui>(ui: &U) -> Result<Option<String>, InteractiveError> {
    ui.println("2. Scope (optional — press Enter to skip):");
    ui.println("   e.g. api, parser, auth-service");
    ui.println("");

    loop {
        let input = ui.prompt("Scope: ").map_err(InteractiveError::Ui)?;

        if input.is_empty() {
            ui.println("");
            return Ok(None);
        }

        match CommitMessage::validate_scope(&input) {
            Ok(()) => {
                ui.println("");
                return Ok(Some(input));
            }
            Err(_) => {
                ui.println("  ✗ Scope must be alphanumeric with hyphens/underscores only.");
            }
        }
    }
}

pub fn collect_description<U: Ui>(ui: &U) -> Result<String, InteractiveError> {
    ui.println("3. Description (max 72 characters):");
    ui.println("");

    loop {
        let input = ui.prompt("Description: ").map_err(InteractiveError::Ui)?;

        if input.is_empty() {
            ui.println("  ✗ Description cannot be empty.");
            continue;
        }

        if input.len() > 72 {
            ui.println(&format!(
                "  ✗ {}/72 characters — too long. Please shorten.",
                input.len()
            ));
            continue;
        }

        ui.println("");
        return Ok(input);
    }
}
