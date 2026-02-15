//! CLI entry point — the composition root for grit.
//!
//! The only file in the codebase that names concrete adapter types.
//! Mode selection happens here. AppController never knows which mode ran.
//!
//! Default (no flags):  editor mode   — opens $EDITOR with template
//! -i / --interactive:  interactive   — guided field-by-field prompts
//! -m / --message:      direct        — inline string (coming next)

use std::process::ExitCode;

use clap::Parser;

use crate::adapters::{GitCommitExecutor, GitStagingChecker, TerminalUI};
use crate::app::AppController;
use crate::compiler::CompilerPipeline;
use crate::input::{EditorSource, InteractiveSource};

#[derive(Parser)]
#[command(
    name = "grit",
    about = "Conventional commit helper",
    long_about = None,
)]
struct Cli {
    /// Open field-by-field interactive prompts instead of editor
    #[arg(short = 'i', long = "interactive")]
    interactive: bool,
}

pub fn run() -> ExitCode {
    let cli = Cli::parse();

    let staging = GitStagingChecker;
    let executor = GitCommitExecutor;
    let ui = TerminalUI;

    if cli.interactive {
        let source = InteractiveSource::new(TerminalUI);
        AppController::new(staging, source, ui, executor).run()
    } else {
        let source = EditorSource::new(CompilerPipeline::new());
        AppController::new(staging, source, ui, executor).run()
    }
}
