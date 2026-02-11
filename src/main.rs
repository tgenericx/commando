mod cli;
mod commit_executor;
mod commit_message;
mod commit_types;
mod compiler;
mod input_collector;
mod staging_checker;

use cli::{CliController, CliMode};
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    // Parse command-line arguments
    let mode = if args.len() > 1 {
        // Check for help flag
        if args[1] == "--help" || args[1] == "-h" {
            print_usage();
            return ExitCode::SUCCESS;
        }

        // Check for message flag
        if args[1] == "--message" || args[1] == "-m" {
            if args.len() < 3 {
                eprintln!("Error: --message flag requires a commit message");
                eprintln!();
                print_usage();
                return ExitCode::FAILURE;
            }

            // Join all remaining args as the message (in case it contains spaces)
            let message = args[2..].join(" ");
            CliMode::Direct { message }
        } else {
            // Unknown flag
            eprintln!("Error: Unknown argument '{}'", args[1]);
            eprintln!();
            print_usage();
            return ExitCode::FAILURE;
        }
    } else {
        // No arguments - use interactive mode
        CliMode::Interactive
    };

    let controller = CliController::with_mode(mode);
    controller.run()
}

fn print_usage() {
    println!("Commando - Conventional Commit Message Tool");
    println!();
    println!("USAGE:");
    println!("    commando [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help              Print this help message");
    println!("    -m, --message <MSG>     Provide commit message directly (non-interactive)");
    println!();
    println!("MODES:");
    println!();
    println!("  Interactive Mode (default):");
    println!("    commando");
    println!("    Guides you through creating a commit message step-by-step.");
    println!();
    println!("  Direct Mode:");
    println!("    commando -m \"feat(api): add new endpoint\"");
    println!("    commando -m \"fix: resolve login bug");
    println!();
    println!("    This is the body of the commit message.");
    println!();
    println!("    BREAKING CHANGE: API endpoint changed\"");
    println!();
    println!("COMMIT MESSAGE FORMAT:");
    println!("    <type>(<scope>): <description>");
    println!();
    println!("    [optional body]");
    println!();
    println!("    [optional footer(s)]");
    println!();
    println!("  Types:");
    println!("    feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert");
    println!();
    println!("  Scope (optional):");
    println!("    Module or area affected (e.g., api, parser, auth)");
    println!();
    println!("  Breaking Changes:");
    println!("    Add '!' after type/scope and include BREAKING CHANGE footer");
    println!("    Example: feat(api)!: change structure");
    println!();
    println!("EXAMPLES:");
    println!();
    println!("  Simple commit:");
    println!("    commando -m \"feat: add user authentication\"");
    println!();
    println!("  With scope:");
    println!("    commando -m \"fix(parser): handle edge case in lexer\"");
    println!();
    println!("  With body:");
    println!("    commando -m \"docs: update README");
    println!();
    println!("    Added installation instructions and usage examples.\"");
    println!();
    println!("  Breaking change:");
    println!("    commando -m \"feat(api)!: redesign authentication");
    println!();
    println!("    Complete overhaul of the auth system.");
    println!();
    println!("    BREAKING CHANGE: Auth tokens now expire after 1 hour\"");
}
