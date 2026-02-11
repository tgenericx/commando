use crate::compiler::error::CompileError;
use crate::compiler::generator::CommitFormatter;
use crate::compiler::lexer::Lexer;
use crate::compiler::parser::Parser;
use crate::compiler::semantic::SemanticAnalyzer;

pub fn compile(input: &str) -> Result<String, CompileError> {
    // 1️⃣ Lexical analysis
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;

    // 2️⃣ Parsing
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;

    // 3️⃣ Semantic validation
    SemanticAnalyzer::analyze(&ast)?;

    // 4️⃣ Code generation
    Ok(CommitFormatter::format(&ast))
}
