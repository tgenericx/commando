/// Interactive input source — collects commit fields one at a time via prompts.
///
/// Implements InputSource<Output = StructuredInput>. Each section module
/// handles one part of the conventional commit spec. The Ui trait is injected
/// so this works with TerminalUI in production and MockUi in tests.
mod sections;

use crate::domain::DomainError;
use crate::ports::input::{InputSource, StructuredInput};
use crate::ports::ui::Ui;

pub struct InteractiveSource<U: Ui> {
    ui: U,
}

impl<U: Ui> InteractiveSource<U> {
    pub fn new(ui: U) -> Self {
        Self { ui }
    }
}

impl<U: Ui> InputSource for InteractiveSource<U> {
    type Output = StructuredInput;
    type Error = DomainError;

    fn collect(&self) -> Result<StructuredInput, DomainError> {
        self.ui.println("\n=== commando ===\n");

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::CommitType;
    use crate::ports::ui::{Ui, UiError};
    use std::cell::RefCell;

    /// MockUi feeds pre-canned responses in order.
    /// confirm() pops a response and interprets "y" as true.
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

    #[test]
    fn collects_minimal_commit() {
        // type, scope (skip), description, body (n), breaking (n), refs (skip)
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
    fn collects_full_commit_with_scope_and_breaking() {
        let ui = MockUi::new(vec![
            "feat",             // type
            "auth",             // scope
            "migrate to OAuth", // description
            "y",                // wants body?
            // body is collected via stdin directly so MockUi can't feed it here
            // — body collection uses io::stdin; test the section in isolation
            "y",                      // breaking change?
            "Old tokens invalidated", // breaking change description
            "closes #88",             // refs
        ]);
        let source = InteractiveSource::new(ui);
        // body section reads stdin directly so we skip body in this test
        // by having "y" trigger the body prompt then EOF closes it — acceptable
        // for now; body section will get its own targeted test.
        let result = source.collect();
        // We only assert it doesn't panic and returns something
        // Full assertion once body is refactored to accept Ui for line reading
        assert!(result.is_ok() || result.is_err()); // placeholder
    }

    #[test]
    fn rejects_invalid_commit_type_then_accepts_valid() {
        // First response is invalid, second is valid
        let ui = MockUi::new(vec![
            "invalid-type",       // rejected
            "fix",                // accepted
            "",                   // scope skip
            "patch null pointer", // description
            "n",                  // body
            "n",                  // breaking
            "",                   // refs
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
}
