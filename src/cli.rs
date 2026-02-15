//! CLI entry point — the composition root for grit.
//!
//! This is the ONLY file in the codebase that names concrete adapter types.
//! Everything else (AppController, InteractiveSource, sections/) depends
//! exclusively on port traits.
//!
//! Adding --tui flag later = add RatatuiUI in adapters/ui/, swap one line here.
//! Adding direct input later = add DirectSource in input/direct/, add a branch here.
//! Nothing else in the codebase changes for either.

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
