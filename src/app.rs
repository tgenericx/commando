/// Application controller — the core orchestration layer.
///
/// Generic over all port traits. Never names a concrete type.
/// After the CommitMessageSource migration, run() no longer knows
/// whether input came from an editor, a CLI arg, or interactive prompts.
///
/// Dependency graph:
///   AppController → ports::{StagingChecker, CommitMessageSource, Ui, CommitExecutor}
///   AppController → domain::CommitMessage (returned by source.resolve())
///   AppController → nothing from adapters/, compiler/, or input/
use std::process::ExitCode;

use crate::ports::{
    executor::{CommitExecutor, DryRunner},
    input::CommitMessageSource,
    staging::StagingChecker,
    ui::Ui,
};

pub struct AppController<S, M, U, E>
where
    S: StagingChecker,
    M: CommitMessageSource,
    U: Ui,
    E: CommitExecutor + DryRunner,
{
    staging: S,
    source: M,
    ui: U,
    executor: E,
}

impl<S, M, U, E> AppController<S, M, U, E>
where
    S: StagingChecker,
    S::Error: std::fmt::Display,
    M: CommitMessageSource,
    M::Error: std::fmt::Display,
    U: Ui,
    E: CommitExecutor + DryRunner,
    <E as CommitExecutor>::Error: std::fmt::Display,
    <E as DryRunner>::Error: std::fmt::Display,
{
    pub fn new(staging: S, source: M, ui: U, executor: E) -> Self {
        Self {
            staging,
            source,
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

        // ── Step 2: resolve input → CommitMessage ─────────────────────
        // One call. Editor, direct, or interactive — AppController doesn't know.
        let message = match self.source.resolve() {
            Ok(m) => m,
            Err(e) => {
                self.ui.println(&format!("Error: {}", e));
                return ExitCode::FAILURE;
            }
        };

        // ── Step 3: preview + confirm ─────────────────────────────────
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

        // ── Step 4: execute ───────────────────────────────────────────
        self.ui.println("\nExecuting git commit...");
        match self.executor.execute(&message.to_conventional_commit()) {
            Ok(result) => {
                self.ui.println(&format!("✓ Committed: {}", result.summary));
                self.ui.println(&format!("  SHA: {}", result.sha));
                ExitCode::SUCCESS
            }
            Err(e) => {
                self.ui.println(&format!("✗ Commit failed: {}", e));
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
    use crate::domain::{CommitMessage, CommitType};
    use crate::ports::{
        executor::{CommitExecutor, CommitResult, DryRunner},
        input::CommitMessageSource,
        staging::StagingChecker,
        ui::{Ui, UiError},
    };
    use std::cell::RefCell;

    // ── mocks ─────────────────────────────────────────────────────────────────

    struct MockStaging(bool);
    impl StagingChecker for MockStaging {
        type Error = String;
        fn has_staged_changes(&self) -> Result<bool, String> {
            Ok(self.0)
        }
    }

    struct MockSource(Result<CommitMessage, String>);
    impl CommitMessageSource for MockSource {
        type Error = String;
        fn resolve(&self) -> Result<CommitMessage, String> {
            self.0.clone()
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

    fn ok_source() -> MockSource {
        MockSource(Ok(CommitMessage::new(
            CommitType::Feat,
            None,
            "add feature".into(),
            None,
            None,
        )
        .unwrap()))
    }

    fn make_app(
        staged: bool,
        confirmed: bool,
        executor_ok: bool,
    ) -> AppController<MockStaging, MockSource, MockUi, MockExecutor> {
        AppController::new(
            MockStaging(staged),
            ok_source(),
            MockUi::new(confirmed),
            MockExecutor {
                succeeds: executor_ok,
            },
        )
    }

    // ── tests ─────────────────────────────────────────────────────────────────

    #[test]
    fn succeeds_end_to_end() {
        assert_eq!(make_app(true, true, true).run(), ExitCode::SUCCESS);
    }

    #[test]
    fn fails_when_no_staged_changes() {
        assert_eq!(make_app(false, true, true).run(), ExitCode::FAILURE);
    }

    #[test]
    fn fails_when_user_aborts_at_confirm() {
        assert_eq!(make_app(true, false, true).run(), ExitCode::FAILURE);
    }

    #[test]
    fn fails_when_executor_fails() {
        assert_eq!(make_app(true, true, false).run(), ExitCode::FAILURE);
    }

    #[test]
    fn fails_when_source_errors() {
        let app = AppController::new(
            MockStaging(true),
            MockSource(Err("editor closed without saving".into())),
            MockUi::new(true),
            MockExecutor { succeeds: true },
        );
        assert_eq!(app.run(), ExitCode::FAILURE);
    }
}
