//! CLI entry point — the composition root for commando.
//!
//! The only file in the codebase that names concrete adapter types.
//! Mode selection happens here. AppController never knows which mode ran.
//!
//! Default (no flags):    editor mode   — opens $EDITOR with template
//! -m / --message <MSG>:  direct mode   — inline string, no editor
//! -i / --interactive:    interactive   — guided field-by-field prompts
//!
//! Multi-line messages with -m:
//!   commando -m $'feat(auth): add OAuth\n\nBody text here.'
//!   commando -m "feat(auth): add OAuth
//!
//! Body text here."

use std::process::ExitCode;

use clap::{ArgGroup, Parser};

use crate::adapters::{GitCommitExecutor, GitStagingChecker, TerminalUI};
use crate::app::AppController;
use crate::compiler::CompilerPipeline;
use crate::input::{DirectSource, EditorSource, InteractiveSource};

#[derive(Parser)]
#[command(
    name = "commando",
    about = "Conventional commit helper",
    long_about = None,
)]
#[command(group(ArgGroup::new("mode").args(["message", "interactive"])))]
struct Cli {
    /// Inline commit message — skips the editor.
    /// Supports multi-line: use $'...\n...' or a quoted newline in your shell.
    #[arg(short = 'm', long = "message", value_name = "MSG")]
    message: Option<String>,

    /// Open field-by-field interactive prompts instead of the editor.
    #[arg(short = 'i', long = "interactive")]
    interactive: bool,
}

pub fn run() -> ExitCode {
    let cli = Cli::parse();

    let staging = GitStagingChecker;
    let executor = GitCommitExecutor;
    let ui = TerminalUI;

    match (cli.message, cli.interactive) {
        (Some(msg), _) => {
            let source = DirectSource::new(msg, CompilerPipeline::new());
            AppController::new(staging, source, ui, executor).run()
        }
        (None, true) => {
            let source = InteractiveSource::new(TerminalUI);
            AppController::new(staging, source, ui, executor).run()
        }
        (None, false) => {
            let source = EditorSource::new(CompilerPipeline::new());
            AppController::new(staging, source, ui, executor).run()
        }
    }
}
