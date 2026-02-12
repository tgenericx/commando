use crate::compiler::compile::compile;
use crate::input::{InputCollector, InputError, InputMode};
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
    interactive_collector: InputCollector,
}

impl CliController {
    pub fn new() -> Self {
        Self {
            staging_checker: StagingChecker::new(),
            editor: Editor::new(),
            message_collector: MessageCollector::new(),
            commit_executor: CommitExecutor::new(),
            interactive_collector: InputCollector::new(),
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
            Command::Interactive => self.run_interactive(),
            Command::Message(msg) => self.run_direct(&msg),
            Command::Editor => self.run_editor(),
        }
    }

    fn run_direct(&self, message: &str) -> Result<(), CliError> {
        println!("📝 Direct message mode");

        // Validate first
        match compile(message) {
            Ok(formatted) => {
                // Check staging
                match self.staging_checker.has_staged_changes() {
                    Ok(true) => println!("✓ Staged changes detected\n"),
                    Ok(false) => return Err(CliError::NoStagedChanges),
                    Err(e) => return Err(CliError::GitError(e)),
                }

                if !self.preview_and_confirm(&formatted) {
                    println!("\nCommit cancelled.");
                    return Ok(());
                }

                match self.commit_executor.execute(&formatted) {
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
            Err(e) => {
                eprintln!("✗ Invalid commit message:");
                eprintln!("  {}", e);
                eprintln!("\nRun with -i for interactive mode or -m alone to open editor.");
                Err(CliError::CompileError(message.to_string()))
            }
        }
    }

    fn run_editor(&self) -> Result<(), CliError> {
        println!("📝 Editor mode (empty -m flag)");

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

    fn run_interactive(&self) -> Result<(), CliError> {
        println!("🔧 Interactive mode");

        // Check staging first
        match self.staging_checker.has_staged_changes() {
            Ok(true) => println!("✓ Staged changes detected\n"),
            Ok(false) => return Err(CliError::NoStagedChanges),
            Err(e) => return Err(CliError::GitError(e)),
        }

        // Collect message via interactive prompts
        let message = self
            .interactive_collector
            .collect(InputMode::Interactive)
            .map_err(|e| match e {
                InputError::Cancelled => CliError::UserCancelled,
                InputError::Empty => CliError::EmptyMessage,
                InputError::Compilation(ce) => CliError::CompileError(ce.to_string()),
                InputError::Io(ioe) => CliError::Io(ioe),
            })?;

        // Preview and confirm
        if !self.preview_and_confirm(&message) {
            println!("\nCommit cancelled.");
            return Ok(());
        }

        // Execute the commit
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
        println!("  commando [OPTIONS]");
        println!();
        println!("OPTIONS:");
        println!("  (none)          Interactive mode - guided prompts");
        println!("  -i, --interactive  Interactive mode (explicit)");
        println!("  -m, --message MSG  Commit with message (validates)");
        println!("  -m, --message      Open editor to write message");
        println!("  --validate MSG     Validate message only");
        println!("  -h, --help         Show this help");
        println!("  -v, --version      Show version");
    }

    fn print_version(&self) {
        println!("commando 0.1.0");
    }
}
