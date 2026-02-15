mod ast;
mod error;
mod lexer;
mod parser;
mod token;

pub use ast::CommitAst;
pub use error::CompileError;

use lexer::Lexer;
use parser::Parser;

/// CompilerPipeline — the public API for the compiler module.
///
/// Callers (DirectSource, EditorSource) only ever touch this type.
/// token, lexer, ast, parser, error are all internal to compiler/.
///
/// compile() is the single entry point:
///   raw string → Lexer → Vec<Token> → Parser → CommitAst
///
/// CommitAst then flows to CommitMessage::try_from(ast) in the domain layer.
#[derive(Debug, Default)]
pub struct CompilerPipeline;

impl CompilerPipeline {
    pub fn new() -> Self {
        Self
    }

    /// Compile a raw commit message string into a CommitAst.
    ///
    /// Returns Err(CompileError) for structural failures only:
    ///   - missing ':' in header
    ///   - empty type or description
    ///   - unclosed scope parenthesis
    ///   - malformed footer syntax
    ///
    /// Does NOT return an error for invalid commit types, long descriptions,
    /// or bad scope characters — those are DomainErrors, not CompileErrors.
    pub fn compile(&self, input: &str) -> Result<CommitAst, CompileError> {
        let tokens = Lexer::new(input).tokenize()?;
        Parser::new(tokens).parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiles_minimal() {
        let ast = CompilerPipeline::new().compile("feat: add login").unwrap();
        assert_eq!(ast.header.commit_type, "feat");
        assert_eq!(ast.header.description, "add login");
    }

    #[test]
    fn compiles_full() {
        let input = "feat(auth)!: migrate to OAuth\n\n\
                     Body text here.\n\n\
                     BREAKING CHANGE: sessions invalidated\n\
                     Refs: #42";
        let ast = CompilerPipeline::new().compile(input).unwrap();
        assert_eq!(ast.header.commit_type, "feat");
        assert_eq!(ast.header.scope, Some("auth".into()));
        assert!(ast.header.breaking);
        assert!(ast.body.is_some());
        assert_eq!(ast.footers.len(), 2);
    }

    #[test]
    fn compile_error_on_missing_colon() {
        let result = CompilerPipeline::new().compile("feat add something");
        assert!(matches!(result, Err(CompileError::Lex(_))));
    }

    #[test]
    fn unknown_type_is_not_a_compile_error() {
        // Semantic validation is the domain's job
        let ast = CompilerPipeline::new()
            .compile("notavalidtype: do something")
            .unwrap();
        assert_eq!(ast.header.commit_type, "notavalidtype");
    }
}
