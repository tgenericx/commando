pub fn strip_comments(content: &str) -> String {
    content
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

    #[test]
    fn strips_comment_lines() {
        let content = r#"feat: add test
# This is a comment
# Another comment

body line
# Footer comment"#;

        let result = strip_comments(content);
        assert_eq!(result, "feat: add test\n\nbody line");
    }

    #[test]
    fn strips_indented_comments() {
        let content = r#"feat: add test
    # indented comment
        # deeply indented
"#;

        let result = strip_comments(content);
        assert_eq!(result, "feat: add test");
    }

    #[test]
    fn returns_empty_for_only_comments() {
        let content = r#"# Only comments
# Another comment
"#;

        let result = strip_comments(content);
        assert_eq!(result, "");
    }
}
