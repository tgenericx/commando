pub mod ast;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod token;

use error::CompileError;

pub fn compile(_input: &str) -> Result<crate::commit_message::CommitMessage, CompileError> {
    Err(CompileError::NotImplemented)
}
