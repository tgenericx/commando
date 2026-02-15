mod error;
pub use error::DirectError;

use crate::compiler::CompilerPipeline;
use crate::domain::CommitMessage;
use crate::ports::input::CommitMessageSource;

/// DirectSource — compiles an inline message string into a CommitMessage.
///
/// The string is provided at construction time from the -m CLI argument.
/// Multi-line messages work naturally: the shell resolves escape sequences
/// before the string reaches grit, so "\n" in the shell becomes a real
/// newline in the String. The compiler already handles multiline input.
///
/// Examples (shell):
///   grit -m "feat: add login"
///   grit -m $'feat(auth): add OAuth\n\nMigrated from sessions.'
///   grit -m "feat(auth): add OAuth
///
/// Migrated from sessions."
///
/// No prompts, no editor, no I/O. resolve() is pure: String → CommitMessage.
pub struct DirectSource {
    raw: String,
    compiler: CompilerPipeline,
}

impl DirectSource {
    pub fn new(raw: String, compiler: CompilerPipeline) -> Self {
        Self { raw, compiler }
    }
}

impl CommitMessageSource for DirectSource {
    type Error = DirectError;

    fn resolve(&self) -> Result<CommitMessage, DirectError> {
        let ast = self.compiler.compile(&self.raw)?;
        CommitMessage::try_from(ast).map_err(DirectError::Domain)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source(raw: &str) -> DirectSource {
        DirectSource::new(raw.to_string(), CompilerPipeline::new())
    }

    // ── valid messages ────────────────────────────────────────────────────────

    #[test]
    fn minimal_single_line() {
        let msg = source("feat: add login").resolve().unwrap();
        assert_eq!(msg.to_conventional_commit(), "feat: add login");
    }

    #[test]
    fn with_scope() {
        let msg = source("fix(auth): correct token expiry").resolve().unwrap();
        assert_eq!(
            msg.to_conventional_commit(),
            "fix(auth): correct token expiry"
        );
    }

    #[test]
    fn with_scope_and_breaking_marker() {
        let msg = source("feat(api)!: remove v1 endpoints").resolve().unwrap();
        assert!(msg.to_conventional_commit().contains("feat(api):"));
    }

    #[test]
    fn multiline_with_body() {
        let raw = "feat: add search\n\nFull-text search using inverted index.";
        let msg = source(raw).resolve().unwrap();
        assert!(msg.to_conventional_commit().contains("feat: add search"));
        assert!(msg.to_conventional_commit().contains("Full-text search"));
    }

    #[test]
    fn multiline_with_breaking_change_footer() {
        let raw = "feat(auth)!: migrate to OAuth\n\nMigrated from sessions.\n\nBREAKING CHANGE: sessions invalidated";
        let msg = source(raw).resolve().unwrap();
        let out = msg.to_conventional_commit();
        assert!(out.starts_with("feat(auth)!:"));
        assert!(out.contains("BREAKING CHANGE: sessions invalidated"));
    }

    #[test]
    fn multiline_with_refs_footer() {
        let raw = "fix: patch null pointer\n\nRefs: #42";
        let msg = source(raw).resolve().unwrap();
        assert!(
            msg.to_conventional_commit()
                .contains("fix: patch null pointer")
        );
    }

    #[test]
    fn full_commit_all_sections() {
        let raw = "feat(auth)!: migrate to OAuth\n\nFull body here.\n\nBREAKING CHANGE: old sessions gone\nRefs: #99";
        let msg = source(raw).resolve().unwrap();
        let out = msg.to_conventional_commit();
        assert!(out.starts_with("feat(auth)!:"));
        assert!(out.contains("Full body here."));
        assert!(out.contains("BREAKING CHANGE: old sessions gone"));
    }

    // ── error cases ───────────────────────────────────────────────────────────

    #[test]
    fn missing_colon_is_compile_error() {
        let result = source("feat add login").resolve();
        assert!(matches!(result, Err(DirectError::Compile(_))));
    }

    #[test]
    fn empty_string_is_compile_error() {
        let result = source("").resolve();
        assert!(matches!(result, Err(DirectError::Compile(_))));
    }

    #[test]
    fn invalid_type_is_domain_error() {
        let result = source("notavalidtype: do something").resolve();
        assert!(matches!(result, Err(DirectError::Domain(_))));
    }

    #[test]
    fn description_too_long_is_domain_error() {
        let long = format!("feat: {}", "a".repeat(73));
        let result = source(&long).resolve();
        assert!(matches!(result, Err(DirectError::Domain(_))));
    }

    #[test]
    fn invalid_scope_is_domain_error() {
        let result = source("feat(invalid scope!): do something").resolve();
        // scope with space and ! fails — either compile error (bad syntax) or domain error
        assert!(result.is_err());
    }
}
