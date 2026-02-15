/// The template written to the temp file before the editor opens.
///
/// Comment lines (starting with #) are stripped after the editor closes.
/// Format follows conventional commits spec.
pub fn commit_template() -> &'static str {
    "\n
# --- grit — conventional commit ---
#
# Format:  type(scope)!: description
#
# Types:   feat  fix  docs  style  refactor  perf  test  build  ci  chore  revert
# Scope:   optional — alphanumeric, hyphens, underscores  e.g. (auth), (api)
# Breaking: add '!' before ':' AND/OR a 'BREAKING CHANGE: ...' footer
#
# --- Examples ---
# feat(auth): add OAuth 2.0 login
#
# Migrated from session-based auth to OAuth 2.0.
# All existing sessions will be invalidated on deploy.
#
# BREAKING CHANGE: session cookies are no longer valid after this release
# Refs: #142
# ---
# Lines starting with '#' are ignored.
# An empty message aborts the commit.
"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_is_non_empty() {
        assert!(!commit_template().is_empty());
    }

    #[test]
    fn template_lines_all_start_with_hash() {
        // Every line in the template is a comment — user starts writing below
        for line in commit_template().lines() {
            if !line.is_empty() {
                assert!(
                    line.starts_with('#'),
                    "Non-comment line in template: '{}'",
                    line
                );
            }
        }
    }

    #[test]
    fn template_mentions_all_types() {
        let t = commit_template();
        for kind in &[
            "feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "ci", "chore",
            "revert",
        ] {
            assert!(t.contains(kind), "template missing type: {}", kind);
        }
    }
}
