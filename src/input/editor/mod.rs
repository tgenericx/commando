mod error;
mod template;

pub use error::EditorError;

use crate::compiler::CompilerPipeline;
use crate::domain::CommitMessage;
use crate::ports::input::CommitMessageSource;
use std::io::Write;
use template::commit_template;

/// EditorSource — opens $EDITOR with a conventional commit template.
///
/// Flow:
///   1. Write template to a named temp file
///   2. Resolve editor: $GIT_EDITOR → $VISUAL → $EDITOR → "vi"
///   3. Spawn the editor process, block until it exits
///   4. Read the file, strip comment lines (lines starting with '#')
///   5. Compile the cleaned content through CompilerPipeline
///   6. Convert CommitAst → CommitMessage via TryFrom (domain validates)
///
/// The editor spawn itself is not unit-testable. Everything else is.
/// See strip_comments() and the tests below.
pub struct EditorSource {
    compiler: CompilerPipeline,
}

impl EditorSource {
    pub fn new(compiler: CompilerPipeline) -> Self {
        Self { compiler }
    }

    fn open_editor(&self) -> Result<String, EditorError> {
        // ── 1. Write template to temp file ───────────────────────────
        let mut temp = tempfile::Builder::new()
            .prefix("grit-")
            .suffix(".txt")
            .tempfile()
            .map_err(|e| EditorError::TempFile(e.to_string()))?;

        temp.write_all(commit_template().as_bytes())
            .map_err(|e| EditorError::TempFile(e.to_string()))?;

        temp.flush()
            .map_err(|e| EditorError::TempFile(e.to_string()))?;

        let path = temp.path().to_owned();

        // ── 2. Resolve editor ────────────────────────────────────────
        let editor = std::env::var("GIT_EDITOR")
            .or_else(|_| std::env::var("VISUAL"))
            .or_else(|_| std::env::var("EDITOR"))
            .unwrap_or_else(|_| "vi".to_string());

        // ── 3. Spawn, block until closed ─────────────────────────────
        let status = std::process::Command::new(&editor)
            .arg(&path)
            .status()
            .map_err(|e| EditorError::SpawnFailed {
                editor: editor.clone(),
                reason: e.to_string(),
            })?;

        if !status.success() {
            return Err(EditorError::EditorFailed(editor));
        }

        // ── 4. Read and strip comments ───────────────────────────────
        let raw = std::fs::read_to_string(&path)
            .map_err(|e| EditorError::ReadFailed(e.to_string()))?;

        let cleaned = strip_comments(&raw);

        if cleaned.is_empty() {
            return Err(EditorError::EmptyMessage);
        }

        Ok(cleaned)
    }
}

impl CommitMessageSource for EditorSource {
    type Error = EditorError;

    fn resolve(&self) -> Result<CommitMessage, EditorError> {
        let raw = self.open_editor()?;
        let ast = self.compiler.compile(&raw)?;
        CommitMessage::try_from(ast).map_err(EditorError::Domain)
    }
}

/// Strip comment lines and trim surrounding whitespace.
///
/// A comment line is any line whose first non-whitespace character is '#'.
/// This mirrors git's own behaviour when editing commit messages.
pub fn strip_comments(input: &str) -> String {
    input
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── strip_comments ────────────────────────────────────────────────────────

    #[test]
    fn strips_pure_comment_file() {
        let input = "# line one\n# line two\n# line three";
        assert_eq!(strip_comments(input), "");
    }

    #[test]
    fn preserves_non_comment_lines() {
        let input = "# grit template\nfeat: add login\n# ignore this";
        assert_eq!(strip_comments(input), "feat: add login");
    }

    #[test]
    fn preserves_multiline_with_comments_interspersed() {
        let input = "feat(auth): add OAuth\n\n# body hint\nFull details here.\n\n# footer hint\nRefs: #42";
        let result = strip_comments(input);
        assert!(result.contains("feat(auth): add OAuth"));
        assert!(result.contains("Full details here."));
        assert!(result.contains("Refs: #42"));
        assert!(!result.contains("# body hint"));
        assert!(!result.contains("# footer hint"));
    }

    #[test]
    fn trims_leading_and_trailing_whitespace() {
        let input = "\n\n# comment\n\nfeat: add login\n\n# end comment\n\n";
        assert_eq!(strip_comments(input), "feat: add login");
    }

    #[test]
    fn empty_input_returns_empty() {
        assert_eq!(strip_comments(""), "");
    }

    #[test]
    fn inline_hash_is_not_a_comment() {
        // '#' not at the start of a line is not a comment
        let input = "fix: resolve #42\n# this is a comment";
        let result = strip_comments(input);
        assert_eq!(result, "fix: resolve #42");
    }

    #[test]
    fn indented_comment_is_stripped() {
        // Lines where # is the first non-whitespace char are comments
        let input = "feat: thing\n  # indented comment\nbody text";
        let result = strip_comments(input);
        assert!(!result.contains("indented comment"));
        assert!(result.contains("body text"));
    }

    // ── full pipeline (strip → compile → domain) ─────────────────────────────
    // These don't require a real editor — they test the logic that runs after
    // the editor closes.

    #[test]
    fn cleaned_content_compiles_to_commit_message() {
        let raw_from_editor = "# grit template\nfeat(auth): add OAuth\n\n# body hint\nMigrated to OAuth.\n\nBREAKING CHANGE: sessions invalidated";
        let cleaned = strip_comments(raw_from_editor);
        let compiler = CompilerPipeline::new();
        let ast = compiler.compile(&cleaned).expect("compile failed");
        let msg = CommitMessage::try_from(ast).expect("domain failed");
        assert!(msg.to_conventional_commit().starts_with("feat(auth)!: add OAuth"));
        assert!(msg.to_conventional_commit().contains("BREAKING CHANGE:"));
    }

    #[test]
    fn domain_error_surfaces_for_invalid_type() {
        let cleaned = "notavalidtype: do something";
        let compiler = CompilerPipeline::new();
        let ast = compiler.compile(cleaned).expect("compile failed");
        let result = CommitMessage::try_from(ast);
        assert!(result.is_err());
    }

    #[test]
    fn compile_error_surfaces_for_bad_structure() {
        let cleaned = "this is not a conventional commit message at all";
        let compiler = CompilerPipeline::new();
        let result = compiler.compile(cleaned);
        assert!(result.is_err());
    }
}
