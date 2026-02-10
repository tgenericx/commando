pub mod ast;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod token;

use error::CompileError;

/// Compile a raw commit message into a validated `CommitMessage`.
///
/// This function orchestrates the compiler pipeline:
/// lexer → parser → semantic analysis → codegen
///
/// NOTE: Pipeline wiring is not yet implemented.
pub fn compile(_input: &str) -> Result<CompilerCommit, CompileError> {
    unimplemented!("Compiler pipeline wiring not implemented yet");
}

/// Internal representation of a commit during compilation.
///
/// This is the form used inside the compiler pipeline.
/// It will later be converted to `CommitMessage` when fully validated.
#[derive(Debug, Clone, Default)]
pub struct CompilerCommit {
    pub type_name: String,
    pub scope: Option<String>,
    pub description: String,
    pub body: Option<String>,
    pub breaking: bool,
    pub footer: Option<String>,
}
