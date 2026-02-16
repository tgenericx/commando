use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
};

pub struct SyntaxHighlighter;

impl SyntaxHighlighter {
    /// Apply syntax highlighting to commit message
    pub fn highlight_commit_message(content: &str) -> Text<'_> {
        let mut lines = Vec::new();
        let content_lines: Vec<&str> = content.lines().collect();

        for (idx, line) in content_lines.iter().enumerate() {
            if idx == 0 {
                lines.push(Self::highlight_header_line(line));
            } else if line.starts_with("BREAKING CHANGE:") {
                let mut spans = vec![Span::styled(
                    "BREAKING CHANGE:",
                    Style::default()
                        .fg(Color::Red)
                        .add_modifier(Modifier::BOLD),
                )];
                if let Some(rest) = line.strip_prefix("BREAKING CHANGE:") {
                    spans.push(Span::styled(rest, Style::default().fg(Color::Red)));
                }
                lines.push(Line::from(spans));
            } else if line.is_empty() {
                lines.push(Line::from(""));
            } else {
                lines.push(Line::from(Span::styled(
                    *line,
                    Style::default().fg(Color::Gray),
                )));
            }
        }

        Text::from(lines)
    }

    /// Highlight the header line (first line) of commit message
    pub fn highlight_header_line(line: &str) -> Line<'_> {
        let mut spans = Vec::new();

        // Parse type, scope, breaking, and description
        if let Some((type_part, rest)) = line.split_once(':') {
            let has_breaking = type_part.contains('!');
            let (type_str, scope_str) = if let Some(idx) = type_part.find('(') {
                let type_only = &type_part[..idx];
                let scope_end = type_part.find(')').unwrap_or(type_part.len());
                let scope = &type_part[idx..=scope_end.min(type_part.len() - 1)];
                (type_only, Some(scope))
            } else {
                (type_part.trim_end_matches('!'), None)
            };

            // Colorize type
            let type_color = Self::type_color(type_str);
            spans.push(Span::styled(
                type_str.to_string(),
                Style::default()
                    .fg(type_color)
                    .add_modifier(Modifier::BOLD),
            ));

            // Add scope if present
            if let Some(scope) = scope_str {
                spans.push(Span::styled(
                    scope.to_string(),
                    Style::default().fg(Color::Magenta),
                ));
            }

            // Add breaking change indicator
            if has_breaking {
                spans.push(Span::styled(
                    "!".to_string(),
                    Style::default()
                        .fg(Color::Red)
                        .add_modifier(Modifier::BOLD),
                ));
            }

            // Add colon
            spans.push(Span::raw(":"));

            // Add description
            spans.push(Span::styled(
                rest.to_string(),
                Style::default().fg(Color::White),
            ));
        } else {
            // No colon found, just render as plain text
            spans.push(Span::styled(
                line.to_string(),
                Style::default().fg(Color::White),
            ));
        }

        Line::from(spans)
    }

    /// Get color for commit type
    fn type_color(type_str: &str) -> Color {
        match type_str {
            "feat" => Color::Green,
            "fix" => Color::Red,
            "docs" => Color::Blue,
            "style" => Color::Yellow,
            "refactor" => Color::Cyan,
            "perf" => Color::Magenta,
            "test" => Color::LightBlue,
            "build" => Color::LightYellow,
            "ci" => Color::LightGreen,
            "chore" => Color::Gray,
            "revert" => Color::LightRed,
            _ => Color::White,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_type() {
        let line = SyntaxHighlighter::highlight_header_line("feat: add feature");
        assert_eq!(line.spans.len(), 3); // type, colon, description
    }

    #[test]
    fn test_type_with_scope() {
        let line = SyntaxHighlighter::highlight_header_line("fix(parser): handle edge case");
        assert_eq!(line.spans.len(), 4); // type, scope, colon, description
    }

    #[test]
    fn test_breaking_change() {
        let line = SyntaxHighlighter::highlight_header_line("feat!: breaking change");
        assert_eq!(line.spans.len(), 4); // type, breaking, colon, description
    }

    #[test]
    fn test_no_colon() {
        let line = SyntaxHighlighter::highlight_header_line("invalid commit message");
        assert_eq!(line.spans.len(), 1); // just plain text
    }
}
