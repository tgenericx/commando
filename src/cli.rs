use std::process::ExitCode;

use clap::{ArgGroup, Parser};

use crate::adapters::{GitCommitExecutor, GitStagingChecker, TerminalUI};
use crate::app::AppController;
use crate::compiler::CompilerPipeline;
use crate::input::{DirectSource, EditorSource, InteractiveSource};

#[cfg(feature = "tui")]
use crate::adapters::ui::RatatuiUI;

#[derive(Parser)]
#[command(
    name = "commando",
    about = "Conventional commit helper with optional rich TUI",
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

    /// Use rich TUI interface (requires 'tui' feature)
    #[cfg(feature = "tui")]
    #[arg(long = "tui")]
    use_tui: bool,
}

pub fn run() -> ExitCode {
    let cli = Cli::parse();

    let staging = GitStagingChecker;
    let executor = GitCommitExecutor;

    // Select UI implementation based on flags and features
    #[cfg(feature = "tui")]
    let use_ratatui = cli.use_tui;

    #[cfg(not(feature = "tui"))]
    let use_ratatui = false;

    match (cli.message, cli.interactive) {
        (Some(msg), _) => {
            // Direct mode - use simple terminal UI
            let source = DirectSource::new(msg, CompilerPipeline::new());
            let ui = TerminalUI;
            AppController::new(staging, source, ui, executor).run()
        }
        (None, true) => {
            // Interactive mode - choose UI based on flag
            if use_ratatui {
                #[cfg(feature = "tui")]
                {
                    let ui = RatatuiUI;
                    let source = InteractiveSource::new(ui);
                    AppController::new(staging, source, ui, executor).run()
                }
                #[cfg(not(feature = "tui"))]
                {
                    eprintln!("Error: --tui flag requires the 'tui' feature to be enabled");
                    eprintln!("Rebuild with: cargo build --features tui");
                    ExitCode::FAILURE
                }
            } else {
                let ui = TerminalUI;
                let source = InteractiveSource::new(ui);
                AppController::new(staging, source, ui, executor).run()
            }
        }
        (None, false) => {
            // Editor mode - use simple terminal UI
            let source = EditorSource::new(CompilerPipeline::new());
            let ui = TerminalUI;
            AppController::new(staging, source, ui, executor).run()
        }
    }
}
