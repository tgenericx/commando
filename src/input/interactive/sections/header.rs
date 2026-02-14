/// Header section — commit type, scope, description.
///
/// Each function validates its field immediately at prompt time.
/// A bad value is rejected before the user moves on — no post-hoc
/// validation needed for these fields.
use crate::domain::{CommitMessage, CommitType, DomainError};
use crate::ports::ui::Ui;

pub fn collect_type<U: Ui>(ui: &U) -> Result<CommitType, DomainError> {
    ui.println("1. Commit type:");
    ui.println("   feat      — new feature");
    ui.println("   fix       — bug fix");
    ui.println("   docs      — documentation only");
    ui.println("   style     — formatting, whitespace");
    ui.println("   refactor  — code restructuring");
    ui.println("   perf      — performance improvement");
    ui.println("   test      — adding or fixing tests");
    ui.println("   build     — build system / dependencies");
    ui.println("   ci        — CI configuration");
    ui.println("   chore     — maintenance");
    ui.println("   revert    — revert a previous commit");
    ui.println("");

    loop {
        let input = ui
            .prompt("Type: ")
            .map_err(|e| DomainError::InvalidCommitType(e.to_string()))?;

        match CommitType::from_str(&input) {
            Ok(ct) => {
                ui.println("");
                return Ok(ct);
            }
            Err(_) => {
                ui.println(&format!(
                    "  ✗ '{}' is not valid. Choose from the list above.",
                    input
                ));
            }
        }
    }
}

pub fn collect_scope<U: Ui>(ui: &U) -> Result<Option<String>, DomainError> {
    ui.println("2. Scope (optional — press Enter to skip):");
    ui.println("   e.g. api, parser, auth-service");
    ui.println("");

    loop {
        let input = ui
            .prompt("Scope: ")
            .map_err(|e| DomainError::InvalidScope(e.to_string()))?;

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

pub fn collect_description<U: Ui>(ui: &U) -> Result<String, DomainError> {
    ui.println("3. Description (max 72 characters):");
    ui.println("");

    loop {
        let input = ui
            .prompt("Description: ")
            .map_err(|_| DomainError::EmptyDescription)?;

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
