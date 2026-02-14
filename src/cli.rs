/// CLI entry point — the composition root.
///
/// This is the only file in the codebase that names concrete adapter types.
/// Everything else depends on port traits. Adding a --tui flag means adding
/// one branch here and a new adapters/ui/ratatui.rs — nothing else changes.
use std::process::ExitCode;

use crate::adapters::{GitCommitExecutor, GitStagingChecker, TerminalUI};
use crate::app::AppController;
use crate::input::InteractiveSource;

pub fn run() -> ExitCode {
    let staging = GitStagingChecker;
    let executor = GitCommitExecutor;
    let ui = TerminalUI;
    let input = InteractiveSource::new(TerminalUI);

    AppController::new(staging, input, ui, executor).run()
}
