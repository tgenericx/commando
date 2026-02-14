/// Application controller — the core orchestration layer.
///
/// Generic over all three port traits. Never names a concrete type.
/// All I/O goes through the injected Ui. All git interaction goes through
/// the injected StagingChecker and CommitExecutor.
///
/// Dependency graph:
///   AppController → ports::{StagingChecker, InputSource, Ui, CommitExecutor}
///   AppController → domain::CommitMessage (via TryFrom)
///   AppController → nothing from adapters/ or input/
use std::process::ExitCode;

use crate::domain::CommitMessage;
use crate::ports::{
    executor::{CommitExecutor, DryRunner},
    input::{InputSource, StructuredInput},
    staging::StagingChecker,
    ui::Ui,
};

pub struct AppController<S, I, U, E>
where
    S: StagingChecker,
    I: InputSource<Output = StructuredInput>,
    U: Ui,
    E: CommitExecutor + DryRunner,
{
    staging: S,
    input: I,
    ui: U,
    executor: E,
}

impl<S, I, U, E> AppController<S, I, U, E>
where
    S: StagingChecker,
    I: InputSource<Output = StructuredInput>,
    I::Error: std::fmt::Display,
    S::Error: std::fmt::Display,
    E: CommitExecutor + DryRunner,
    <E as CommitExecutor>::Error: std::fmt::Display,
    <E as DryRunner>::Error: std::fmt::Display,
    U: Ui,
{
    pub fn new(staging: S, input: I, ui: U, executor: E) -> Self {
        Self {
            staging,
            input,
            ui,
            executor,
        }
    }

    pub fn run(&self) -> ExitCode {
        // ── Step 1: staged changes ────────────────────────────────────
        self.ui.println("Checking for staged changes...");
        match self.staging.has_staged_changes() {
            Ok(true) => self.ui.println("✓ Staged changes detected\n"),
            Ok(false) => {
                self.ui.println("✗ No staged changes found.\n");
                self.ui.println("Stage your changes first:");
                self.ui.println("  git add <files>\n");
                return ExitCode::FAILURE;
            }
            Err(e) => {
                self.ui.println(&format!("Error checking staging: {}", e));
                return ExitCode::FAILURE;
            }
        }

        // ── Step 2: collect input ─────────────────────────────────────
        let structured = match self.input.collect() {
            Ok(s) => s,
            Err(e) => {
                self.ui.println(&format!("Error collecting input: {}", e));
                return ExitCode::FAILURE;
            }
        };

        // ── Step 3: build CommitMessage ───────────────────────────────
        let message = match CommitMessage::try_from(structured) {
            Ok(m) => m,
            Err(e) => {
                self.ui.println(&format!("Validation error: {}", e));
                return ExitCode::FAILURE;
            }
        };

        // ── Step 4: preview + confirm ─────────────────────────────────
        self.ui.show_preview(&message.to_conventional_commit());

        match self.ui.confirm("Proceed with commit?") {
            Ok(true) => {}
            Ok(false) => {
                self.ui.println("\nCommit aborted.");
                return ExitCode::FAILURE;
            }
            Err(e) => {
                self.ui.println(&format!("Error: {}", e));
                return ExitCode::FAILURE;
            }
        }

        // ── Step 5: execute ───────────────────────────────────────────
        self.ui.println("\nExecuting git commit...");
        match self.executor.execute(&message.to_conventional_commit()) {
            Ok(result) => {
                self.ui.println(&format!("✓ Committed: {}", result.summary));
                self.ui.println(&format!("  SHA: {}", result.sha));
                ExitCode::SUCCESS
            }
            Err(e) => {
                self.ui.println(&format!("✗ Commit failed: {}", e));

                // Offer dry-run diagnosis
                if let Ok(true) = self.ui.confirm("Try a dry-run to diagnose?") {
                    match self.executor.dry_run(&message.to_conventional_commit()) {
                        Ok(_) => self.ui.println("Dry-run succeeded. Check your git config."),
                        Err(e) => self.ui.println(&format!("Dry-run also failed: {}", e)),
                    }
                }

                ExitCode::FAILURE
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::CommitType;
    use crate::ports::{
        executor::{CommitExecutor, CommitResult, DryRunner},
        input::{InputSource, StructuredInput},
        staging::StagingChecker,
        ui::{Ui, UiError},
    };
    use std::cell::RefCell;

    // ── mock implementations ──────────────────────────────────────────

    struct MockStaging(bool);
    impl StagingChecker for MockStaging {
        type Error = String;
        fn has_staged_changes(&self) -> Result<bool, String> {
            Ok(self.0)
        }
    }

    struct MockInput(StructuredInput);
    impl InputSource for MockInput {
        type Output = StructuredInput;
        type Error = String;
        fn collect(&self) -> Result<StructuredInput, String> {
            Ok(self.0.clone())
        }
    }

    struct MockUi {
        confirmed: bool,
        output: RefCell<Vec<String>>,
    }
    impl MockUi {
        fn new(confirmed: bool) -> Self {
            Self {
                confirmed,
                output: RefCell::new(vec![]),
            }
        }
    }
    impl Ui for MockUi {
        fn prompt(&self, _: &str) -> Result<String, UiError> {
            Ok(String::new())
        }
        fn show_preview(&self, _: &str) {}
        fn confirm(&self, _: &str) -> Result<bool, UiError> {
            Ok(self.confirmed)
        }
        fn println(&self, msg: &str) {
            self.output.borrow_mut().push(msg.to_string());
        }
    }

    struct MockExecutor {
        succeeds: bool,
    }
    impl CommitExecutor for MockExecutor {
        type Error = String;
        fn execute(&self, msg: &str) -> Result<CommitResult, String> {
            if self.succeeds {
                Ok(CommitResult {
                    sha: "abc123".into(),
                    summary: msg.lines().next().unwrap_or("").into(),
                })
            } else {
                Err("git process failed".into())
            }
        }
    }
    impl DryRunner for MockExecutor {
        type Error = String;
        fn dry_run(&self, _: &str) -> Result<(), String> {
            Ok(())
        }
    }

    fn minimal_input() -> StructuredInput {
        StructuredInput {
            commit_type: CommitType::Feat,
            scope: None,
            description: "add feature".into(),
            body: None,
            breaking_change: None,
            refs: None,
        }
    }

    fn make_app(
        staged: bool,
        confirmed: bool,
        executor_ok: bool,
    ) -> AppController<MockStaging, MockInput, MockUi, MockExecutor> {
        AppController::new(
            MockStaging(staged),
            MockInput(minimal_input()),
            MockUi::new(confirmed),
            MockExecutor {
                succeeds: executor_ok,
            },
        )
    }

    // ── tests ─────────────────────────────────────────────────────────

    #[test]
    fn succeeds_end_to_end() {
        let app = make_app(true, true, true);
        assert_eq!(app.run(), ExitCode::SUCCESS);
    }

    #[test]
    fn fails_when_no_staged_changes() {
        let app = make_app(false, true, true);
        assert_eq!(app.run(), ExitCode::FAILURE);
    }

    #[test]
    fn fails_when_user_aborts_at_confirm() {
        let app = make_app(true, false, true);
        assert_eq!(app.run(), ExitCode::FAILURE);
    }

    #[test]
    fn fails_when_executor_fails() {
        let app = make_app(true, true, false);
        assert_eq!(app.run(), ExitCode::FAILURE);
    }
}
