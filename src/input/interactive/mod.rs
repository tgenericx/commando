mod error;
pub use error::InteractiveError;

/// Interactive input source — collects commit fields one at a time via prompts.
///
/// Implements both InputSource (internal collection) and CommitMessageSource
/// (the unified trait AppController depends on).
///
/// The Ui trait is injected so this works with TerminalUI in production
/// and MockUi in tests. collect() and all sections/ are unchanged.
mod sections;

use crate::domain::CommitMessage;
use crate::ports::input::{CommitMessageSource, InputSource, StructuredInput};
use crate::ports::ui::Ui;

pub struct InteractiveSource<U: Ui> {
    ui: U,
}

impl<U: Ui> InteractiveSource<U> {
    pub fn new(ui: U) -> Self {
        Self { ui }
    }
}

/// Low-level field-by-field collection — unchanged.
/// Still used by resolve() below and by tests.
impl<U: Ui> InputSource for InteractiveSource<U> {
    type Output = StructuredInput;
    type Error = InteractiveError;

    fn collect(&self) -> Result<StructuredInput, InteractiveError> {
        self.ui.println("\n=== grit ===\n");

        let commit_type = sections::header::collect_type(&self.ui)?;
        let scope = sections::header::collect_scope(&self.ui)?;
        let description = sections::header::collect_description(&self.ui)?;
        let body = sections::body::collect(&self.ui)?;
        let breaking_change = sections::footer::collect_breaking_change(&self.ui)?;
        let refs = sections::footer::collect_refs(&self.ui)?;

        Ok(StructuredInput {
            commit_type,
            scope,
            description,
            body,
            breaking_change,
            refs,
        })
    }
}

/// Unified trait impl — what AppController calls.
///
/// Wraps collect() and TryFrom. No changes to sections/.
/// InteractiveError already has a Domain variant from the existing error.rs.
impl<U: Ui> CommitMessageSource for InteractiveSource<U> {
    type Error = InteractiveError;

    fn resolve(&self) -> Result<CommitMessage, InteractiveError> {
        let structured = self.collect()?;
        CommitMessage::try_from(structured).map_err(InteractiveError::Domain)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::CommitType;
    use crate::ports::ui::{Ui, UiError};
    use std::cell::RefCell;

    struct MockUi {
        responses: RefCell<Vec<String>>,
    }

    impl MockUi {
        fn new(responses: Vec<&str>) -> Self {
            Self {
                responses: RefCell::new(responses.iter().map(|s| s.to_string()).collect()),
            }
        }

        fn pop(&self) -> String {
            self.responses
                .borrow_mut()
                .drain(..1)
                .next()
                .unwrap_or_default()
        }
    }

    impl Ui for MockUi {
        fn prompt(&self, _label: &str) -> Result<String, UiError> {
            Ok(self.pop())
        }
        fn confirm(&self, _msg: &str) -> Result<bool, UiError> {
            Ok(matches!(self.pop().to_lowercase().as_str(), "y" | "yes"))
        }
        fn show_preview(&self, _content: &str) {}
        fn println(&self, _msg: &str) {}
    }

    // ── existing collect() tests — all unchanged ──────────────────────────────

    #[test]
    fn collects_minimal_commit() {
        let ui = MockUi::new(vec!["feat", "", "add login page", "n", "n", ""]);
        let source = InteractiveSource::new(ui);
        let result = source.collect().unwrap();
        assert_eq!(result.commit_type, CommitType::Feat);
        assert_eq!(result.scope, None);
        assert_eq!(result.description, "add login page");
        assert_eq!(result.body, None);
        assert_eq!(result.breaking_change, None);
        assert_eq!(result.refs, None);
    }

    #[test]
    fn rejects_invalid_commit_type_then_accepts_valid() {
        let ui = MockUi::new(vec![
            "invalid-type",
            "fix",
            "",
            "patch null pointer",
            "n",
            "n",
            "",
        ]);
        let source = InteractiveSource::new(ui);
        let result = source.collect().unwrap();
        assert_eq!(result.commit_type, CommitType::Fix);
    }

    #[test]
    fn collects_with_scope_and_refs() {
        let ui = MockUi::new(vec![
            "docs",
            "readme",
            "update installation guide",
            "n",
            "n",
            "#42",
        ]);
        let source = InteractiveSource::new(ui);
        let result = source.collect().unwrap();
        assert_eq!(result.commit_type, CommitType::Docs);
        assert_eq!(result.scope, Some("readme".to_string()));
        assert_eq!(result.refs, Some("#42".to_string()));
    }

    #[test]
    fn collects_with_breaking_change() {
        let ui = MockUi::new(vec![
            "feat",
            "auth",
            "migrate to OAuth",
            "n",
            "y",
            "old tokens are invalidated",
            "",
        ]);
        let source = InteractiveSource::new(ui);
        let result = source.collect().unwrap();
        assert_eq!(result.commit_type, CommitType::Feat);
        assert_eq!(result.scope, Some("auth".to_string()));
        assert_eq!(
            result.breaking_change,
            Some("old tokens are invalidated".to_string())
        );
    }

    // ── resolve() tests ───────────────────────────────────────────────────────

    #[test]
    fn resolve_returns_commit_message() {
        let ui = MockUi::new(vec!["feat", "", "add login page", "n", "n", ""]);
        let source = InteractiveSource::new(ui);
        let result = source.resolve().unwrap();
        assert_eq!(result.to_conventional_commit(), "feat: add login page");
    }

    #[test]
    fn resolve_with_scope_and_breaking() {
        let ui = MockUi::new(vec![
            "feat",
            "auth",
            "migrate to OAuth",
            "n",
            "y",
            "sessions invalidated",
            "",
        ]);
        let source = InteractiveSource::new(ui);
        let msg = source.resolve().unwrap();
        assert!(msg.to_conventional_commit().contains("feat(auth)!:"));
        assert!(msg.to_conventional_commit().contains("BREAKING CHANGE:"));
    }
}
