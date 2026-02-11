use crate::compiler::compile::compile;
use std::process::ExitCode;

use crate::editor::Editor;
use crate::message::MessageCollector;
use crate::staging_checker::StagingChecker;

use self::command::Command;

mod command;
mod error;

use crate::git::CommitExecutor;
pub use error::CliError;

#[derive(Default)]
pub struct CliController {
    staging_checker: StagingChecker,
    editor: Editor,
    message_collector: MessageCollector,
    commit_executor: CommitExecutor,
}

impl CliController {
    pub fn new() -> Self {
        Self {
            staging_checker: StagingChecker::new(),
            editor: Editor::new(),
            message_collector: MessageCollector::new(),
            commit_executor: CommitExecutor::new(),
        }
    }

    fn run_commit(&self) -> Result<(), CliError> {
        println!("Checking staged changes...");

        match self.staging_checker.has_staged_changes() {
            Ok(true) => println!("✓ Staged changes detected\n"),
            Ok(false) => return Err(CliError::NoStagedChanges),
            Err(e) => return Err(CliError::GitError(e)),
        }

        let message = self.message_collector.collect_from_editor(&self.editor)?;

        if !self.preview_and_confirm(&message) {
            println!("\nCommit cancelled.");
            return Ok(());
        }

        // Execute the actual commit
        match self.commit_executor.execute(&message) {
            Ok(result) => {
                println!("\n✓ Commit successful!");
                println!("  SHA: {}", result.sha);
                println!("  {}", result.summary);
                Ok(())
            }
            Err(e) => {
                eprintln!("\n✗ Commit failed: {}", e);
                Err(e)
            }
        }
    }

    pub fn run(&self) -> ExitCode {
        match self.try_run() {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("✗ {}", e);
                ExitCode::FAILURE
            }
        }
    }

    fn try_run(&self) -> Result<(), CliError> {
        let args: Vec<String> = std::env::args().collect();

        match command::parse(&args) {
            Command::Help => {
                self.print_help();
                Ok(())
            }
            Command::Version => {
                self.print_version();
                Ok(())
            }
            Command::Validate(msg) => self.validate_only(&msg),
            Command::Commit => self.run_commit(),
        }
    }

    fn validate_only(&self, message: &str) -> Result<(), CliError> {
        println!("🔍 Validating: \"{}\"", message);

        match compile(message) {
            Ok(formatted) => {
                println!("✓ Valid commit message");
                println!("\nFormatted:\n{}", formatted);
                Ok(())
            }
            Err(e) => {
                eprintln!("✗ {}", e);
                Err(CliError::CompileError(message.to_string()))
            }
        }
    }

    fn preview_and_confirm(&self, message: &str) -> bool {
        println!("\n=== Commit Message Preview ===\n");
        println!("{}", message);
        println!("\n{}", "=".repeat(50));
        println!("\nProceed with commit? (Y/n)");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).ok();

        let choice = choice.trim().to_lowercase();
        choice.is_empty() || choice == "y" || choice == "yes"
    }

    fn print_help(&self) {
        println!("commando - Conventional Commit Tool");
        println!();
        println!("USAGE:");
        println!("  commando [COMMAND]");
        println!();
        println!("COMMANDS:");
        println!("  (none)          Opens editor for message input (default)");
        println!("  --validate MSG  Validate a commit message");
        println!("  --help, -h      Show this help");
        println!("  --version, -v   Show version");
    }

    fn print_version(&self) {
        println!("commando 0.1.0");
    }
}
