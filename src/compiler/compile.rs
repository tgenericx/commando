use crate::compiler::ast::CommitAst;
use crate::compiler::error::CompileError;
use crate::compiler::generator::CommitFormatter;
use crate::compiler::lexer::Lexer;
use crate::compiler::parser::Parser;
use crate::compiler::semantic::SemanticAnalyzer;

fn validate_and_get_ast(input: &str) -> Result<CommitAst, CompileError> {
    let tokens = Lexer::new(input).tokenize()?;
    let ast: CommitAst = Parser::new(tokens).parse()?;
    SemanticAnalyzer::analyze(&ast)?;
    Ok(ast)
}

pub fn validate(input: &str) -> Result<(), CompileError> {
    validate_and_get_ast(input).map(|_| ())
}

/// Compile a raw commit message into a validated Commit.
pub fn compile(input: &str) -> Result<String, CompileError> {
    let ast = validate_and_get_ast(input)?;
    Ok(CommitFormatter::format(&ast))
}
