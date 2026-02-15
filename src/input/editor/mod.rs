mod error;
mod template;

pub use error::EditorError;

use std::io::Write;
use std::path::{Path, PathBuf};

use crate::compiler::CompilerPipeline;
use crate::domain::CommitMessage;
use crate::ports::input::CommitMessageSource;
use template::commit_template;

// ── File lifecycle ────────────────────────────────────────────────────────────

/// RAII guard — deletes the file when dropped.
///
/// Keeps cleanup in one place. On success or final abort, dropping this
/// guard removes the file. The path is stable across all retry iterations
/// so the user's content is never lost between opens.
struct TempCommitFile {
    path: PathBuf,
}

impl TempCommitFile {
    /// Create the file and write the initial template to it.
    fn create() -> Result<Self, EditorError> {
        let path = std::env::temp_dir().join(format!("grit-{}.txt", std::process::id()));
        let mut file =
            std::fs::File::create(&path).map_err(|e| EditorError::TempFile(e.to_string()))?;
        file.write_all(commit_template().as_bytes())
            .map_err(|e| EditorError::TempFile(e.to_string()))?;
        Ok(Self { path })
    }

    fn path(&self) -> &Path {
        &self.path
    }

    /// Overwrite the file with new content (used to inject error comments).
    fn write(&self, content: &str) -> Result<(), EditorError> {
        std::fs::write(&self.path, content).map_err(|e| EditorError::TempFile(e.to_string()))
    }

    fn read(&self) -> Result<String, EditorError> {
        std::fs::read_to_string(&self.path).map_err(|e| EditorError::ReadFailed(e.to_string()))
    }
}

impl Drop for TempCommitFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

// ── Editor resolution ─────────────────────────────────────────────────────────

fn resolve_editor() -> String {
    std::env::var("GIT_EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .or_else(|_| std::env::var("EDITOR"))
        .unwrap_or_else(|_| "vi".to_string())
}

fn spawn_editor(editor: &str, path: &Path) -> Result<(), EditorError> {
    let status = std::process::Command::new(editor)
        .arg(path)
        .status()
        .map_err(|e| EditorError::SpawnFailed {
            editor: editor.to_string(),
            reason: e.to_string(),
        })?;

    if !status.success() {
        return Err(EditorError::EditorFailed(editor.to_string()));
    }
    Ok(())
}

// ── Comment handling ──────────────────────────────────────────────────────────

/// Strip comment lines and trim surrounding whitespace.
/// A comment line is any line whose first non-whitespace character is '#'.
pub fn strip_comments(input: &str) -> String {
    input
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

/// Prepend an error as a comment block above the user's existing content.
///
/// The user re-opens the file and sees exactly what went wrong at the top,
/// with their message intact below. They fix it and save — no content lost.
fn inject_error_comment(error: &str, existing_content: &str) -> String {
    let error_block = error
        .lines()
        .map(|l| format!("# ERROR: {}", l))
        .collect::<Vec<_>>()
        .join("\n");

    format!("{}\n#\n{}", error_block, existing_content)
}

// ── EditorSource ──────────────────────────────────────────────────────────────

/// EditorSource — opens $EDITOR with a conventional commit template.
///
/// On validation error, the user is asked whether they want to fix it.
/// If yes: the error is injected as a comment above their content and the
/// editor reopens with the file intact. If no: the commit is aborted.
/// The temp file is cleaned up automatically when EditorSource drops.
pub struct EditorSource {
    compiler: CompilerPipeline,
}

impl EditorSource {
    pub fn new(compiler: CompilerPipeline) -> Self {
        Self { compiler }
    }
}

impl CommitMessageSource for EditorSource {
    type Error = EditorError;

    fn resolve(&self) -> Result<CommitMessage, EditorError> {
        let editor = resolve_editor();
        let file = TempCommitFile::create()?;

        loop {
            // ── Open editor ───────────────────────────────────────────
            spawn_editor(&editor, file.path())?;

            // ── Read + strip comments ─────────────────────────────────
            let raw = file.read()?;
            let cleaned = strip_comments(&raw);

            if cleaned.is_empty() {
                // Ask whether to retry or abort
                if prompt_retry("Commit message is empty (nothing was written).")? {
                    // Reset file to template and loop
                    file.write(commit_template())?;
                    continue;
                } else {
                    return Err(EditorError::Aborted);
                }
            }

            // ── Compile + domain validate ─────────────────────────────
            let result = self
                .compiler
                .compile(&cleaned)
                .map_err(EditorError::Compile)
                .and_then(|ast| CommitMessage::try_from(ast).map_err(EditorError::Domain));

            match result {
                Ok(message) => return Ok(message),
                Err(e) => {
                    let error_msg = e.to_string();
                    if prompt_retry(&format!("Validation error: {}", error_msg))? {
                        // Inject the error as a comment above the user's content
                        // so they can see what's wrong without losing their work.
                        let annotated = inject_error_comment(&error_msg, &raw);
                        file.write(&annotated)?;
                        continue;
                    } else {
                        return Err(EditorError::Aborted);
                    }
                }
            }
        }
    }
}

/// Ask the user whether to re-open the editor.
///
/// Prints the reason and prompts "(e)dit / (a)bort". Returns true to retry.
/// Reads directly from stdin — this is intentionally outside the Ui trait
/// because EditorSource predates the AppController confirm flow and runs
/// before AppController sees a CommitMessage.
fn prompt_retry(reason: &str) -> Result<bool, EditorError> {
    use std::io::BufRead;

    eprintln!("\n{}", reason);
    eprint!("  (e)dit again / (a)bort [e]: ");

    std::io::stderr().flush().ok();

    let line = std::io::BufReader::new(std::io::stdin())
        .lines()
        .next()
        .transpose()
        .map_err(|e| EditorError::TempFile(e.to_string()))?
        .unwrap_or_default();

    Ok(matches!(
        line.trim().to_lowercase().as_str(),
        "e" | "" | "edit"
    ))
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── strip_comments ────────────────────────────────────────────────────────

    #[test]
    fn strips_pure_comment_file() {
        assert_eq!(strip_comments("# line one\n# line two"), "");
    }

    #[test]
    fn preserves_non_comment_lines() {
        let input = "# grit template\nfeat: add login\n# ignore this";
        assert_eq!(strip_comments(input), "feat: add login");
    }

    #[test]
    fn preserves_multiline_with_comments_interspersed() {
        let input = "feat(auth): add OAuth\n\n# body hint\nDetails.\n\n# footer\nRefs: #42";
        let result = strip_comments(input);
        assert!(result.contains("feat(auth): add OAuth"));
        assert!(result.contains("Details."));
        assert!(result.contains("Refs: #42"));
        assert!(!result.contains("# body hint"));
    }

    #[test]
    fn trims_surrounding_whitespace() {
        let input = "\n\n# comment\n\nfeat: add login\n\n# end\n\n";
        assert_eq!(strip_comments(input), "feat: add login");
    }

    #[test]
    fn empty_input_returns_empty() {
        assert_eq!(strip_comments(""), "");
    }

    #[test]
    fn inline_hash_is_not_a_comment() {
        let input = "fix: resolve #42\n# this is a comment";
        assert_eq!(strip_comments(input), "fix: resolve #42");
    }

    #[test]
    fn indented_comment_is_stripped() {
        let input = "feat: thing\n  # indented comment\nbody text";
        let result = strip_comments(input);
        assert!(!result.contains("indented comment"));
        assert!(result.contains("body text"));
    }

    // ── inject_error_comment ──────────────────────────────────────────────────

    #[test]
    fn injects_error_above_content() {
        let result = inject_error_comment("bad type 'xyz'", "xyz: do something");
        assert!(result.starts_with("# ERROR:"));
        assert!(result.contains("bad type 'xyz'"));
        assert!(result.contains("xyz: do something"));
        // Error appears BEFORE content
        assert!(result.find("# ERROR:").unwrap() < result.find("xyz: do something").unwrap());
    }

    #[test]
    fn injected_error_is_stripped_by_strip_comments() {
        let annotated = inject_error_comment("some error", "feat: fix thing");
        let stripped = strip_comments(&annotated);
        assert_eq!(stripped, "feat: fix thing");
    }

    #[test]
    fn multiline_error_each_line_gets_prefix() {
        let result = inject_error_comment("line one\nline two", "feat: ok");
        assert!(result.contains("# ERROR: line one"));
        assert!(result.contains("# ERROR: line two"));
    }

    // ── full pipeline (no editor spawn) ──────────────────────────────────────

    #[test]
    fn cleaned_content_compiles_to_commit_message() {
        let raw = "# template\nfeat(auth): add OAuth\n\n# hint\nMigrated.\n\nBREAKING CHANGE: sessions invalidated";
        let cleaned = strip_comments(raw);
        let ast = CompilerPipeline::new().compile(&cleaned).unwrap();
        let msg = CommitMessage::try_from(ast).unwrap();
        assert!(msg.to_conventional_commit().starts_with("feat(auth)!:"));
        assert!(msg.to_conventional_commit().contains("BREAKING CHANGE:"));
    }

    #[test]
    fn domain_error_surfaces_for_invalid_type() {
        let ast = CompilerPipeline::new()
            .compile("notavalidtype: do something")
            .unwrap();
        assert!(CommitMessage::try_from(ast).is_err());
    }

    #[test]
    fn compile_error_for_bad_structure() {
        assert!(
            CompilerPipeline::new()
                .compile("not a conventional commit at all")
                .is_err()
        );
    }

    // ── TempCommitFile ────────────────────────────────────────────────────────

    #[test]
    fn temp_file_is_created_with_template_content() {
        let file = TempCommitFile::create().unwrap();
        let content = file.read().unwrap();
        assert!(content.contains("grit"));
        assert!(content.lines().all(|l| l.is_empty() || l.starts_with('#')));
    }

    #[test]
    fn temp_file_write_and_read_roundtrip() {
        let file = TempCommitFile::create().unwrap();
        file.write("feat: test content").unwrap();
        assert_eq!(file.read().unwrap(), "feat: test content");
    }

    #[test]
    fn temp_file_is_deleted_on_drop() {
        let path = {
            let file = TempCommitFile::create().unwrap();
            let p = file.path().to_owned();
            assert!(p.exists());
            p
        }; // file dropped here
        assert!(!path.exists());
    }
}
