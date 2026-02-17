//! Simple commit message syntax highlighter using domain types

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::domain::CommitType;

pub struct CommitHighlighter;

impl CommitHighlighter {
    pub fn highlight(content: &str) -> Vec<Line<'_>> {
        let mut lines = Vec::new();
        let mut in_body = false;
        let mut potential_footers = Vec::new();

        let all_lines: Vec<&str> = content.lines().collect();

        for (i, line) in all_lines.iter().enumerate() {
            if i == 0 {
                lines.push(Self::highlight_subject(line));
                in_body = true;
            } else if line.is_empty() {
                lines.push(Line::from(""));
                potential_footers.clear();
            } else if in_body {
                if Self::looks_like_footer(line) {
                    potential_footers.push(*line);
                } else {
                    if !potential_footers.is_empty() {
                        for footer_line in potential_footers.drain(..) {
                            lines.push(Self::highlight_footer(footer_line));
                        }
                    }
                    lines.push(Self::highlight_body(line));
                }
            }
        }

        for footer_line in potential_footers {
            lines.push(Self::highlight_footer(footer_line));
        }

        lines
    }

    fn highlight_subject(line: &str) -> Line<'_> {
        let mut spans = Vec::new();

        // Check for conventional commit format (type(scope)!: subject)
        if let Some(idx) = line.find(':') {
            let prefix = &line[..idx];
            let subject = &line[idx + 1..].trim_start();

            let has_breaking = prefix.contains('!');

            if prefix.contains('(') && prefix.contains(')') {
                if let (Some(scope_start), Some(scope_end)) = (prefix.find('('), prefix.find(')')) {
                    let type_str = &prefix[..scope_start];
                    let scope = &prefix[scope_start + 1..scope_end];

                    // Parse the commit type using domain model
                    if let Ok(commit_type) = CommitType::from_str(type_str) {
                        spans.push(Self::styled_domain_type(&commit_type));
                    } else {
                        spans.push(Span::styled(
                            type_str.to_string(),
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ));
                    }

                    spans.push(Span::styled(
                        format!("({})", scope),
                        Style::default().fg(Color::Blue),
                    ));

                    if has_breaking {
                        spans.push(Span::styled(
                            "!",
                            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                        ));
                    }

                    spans.push(Span::styled(":", Style::default().fg(Color::DarkGray)));
                }
            } else {
                let clean_type = prefix.replace('!', "");

                // Parse the commit type using domain model
                if let Ok(commit_type) = CommitType::from_str(&clean_type) {
                    spans.push(Self::styled_domain_type(&commit_type));
                } else {
                    spans.push(Span::styled(
                        clean_type,
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ));
                }

                if has_breaking {
                    spans.push(Span::styled(
                        "!",
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    ));
                }

                spans.push(Span::styled(":", Style::default().fg(Color::DarkGray)));
            }

            spans.push(Span::styled(
                format!(" {}", subject),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ));
        } else {
            spans.push(Span::styled(
                line.to_string(),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ));
        }

        Line::from(spans)
    }

    fn styled_domain_type(commit_type: &CommitType) -> Span<'static> {
        let color = match commit_type {
            CommitType::Feat => Color::Green,
            CommitType::Fix => Color::Red,
            CommitType::Docs => Color::Blue,
            CommitType::Style => Color::Cyan,
            CommitType::Refactor => Color::Yellow,
            CommitType::Perf => Color::Magenta,
            CommitType::Test => Color::LightCyan,
            CommitType::Build => Color::Gray,
            CommitType::Ci => Color::LightBlue,
            CommitType::Chore => Color::DarkGray,
            CommitType::Revert => Color::LightRed,
        };

        Span::styled(
            commit_type.as_str().to_string(),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        )
    }

    fn highlight_body(line: &str) -> Line<'_> {
        let mut spans = Vec::new();
        let mut remaining = line;

        while !remaining.is_empty() {
            // Fix 1: Use array of chars instead of closure
            if let Some(start) = remaining.find(['#', '@']) {
                if start > 0 {
                    spans.push(Span::raw(remaining[..start].to_string()));
                }

                let token = &remaining[start..];

                // Fix 2 & 3: Use strip_prefix instead of manual slicing
                if let Some(stripped) = token.strip_prefix('#') {
                    let (num, rest) = Self::extract_digits(stripped);
                    spans.push(Span::styled(
                        format!("#{}", num),
                        Style::default().fg(Color::Yellow),
                    ));
                    remaining = rest;
                } else if let Some(stripped) = token.strip_prefix('@') {
                    let (username, rest) = Self::extract_username(stripped);
                    spans.push(Span::styled(
                        format!("@{}", username),
                        Style::default().fg(Color::Green),
                    ));
                    remaining = rest;
                } else {
                    spans.push(Span::raw(token[..1].to_string()));
                    remaining = &token[1..];
                }
            } else {
                spans.push(Span::raw(remaining.to_string()));
                remaining = "";
            }
        }

        Line::from(spans)
    }

    fn looks_like_footer(line: &str) -> bool {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            return false;
        }

        // Check for key-value pattern with colon
        if let Some(idx) = trimmed.find(':') {
            let key = &trimmed[..idx].trim();
            let value = &trimmed[idx + 1..].trim();

            if key.len() > 30 || value.is_empty() {
                return false;
            }

            let first_char = key.chars().next().unwrap_or(' ');
            if !first_char.is_ascii_alphabetic() {
                return false;
            }

            if !key
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == ' ')
            {
                return false;
            }

            return true;
        }

        // Check for URL patterns
        if trimmed.starts_with("https://") || trimmed.starts_with("http://") {
            return true;
        }

        // Check for issue reference patterns
        let lower = trimmed.to_lowercase();
        if lower.starts_with("fixes")
            || lower.starts_with("closes")
            || lower.starts_with("resolves")
            || lower.starts_with("see")
        {
            return true;
        }

        false
    }

    fn highlight_footer(line: &str) -> Line<'_> {
        // Added <'_>
        let trimmed = line.trim();

        // Try to split at first colon
        if let Some(idx) = trimmed.find(':') {
            let key = &trimmed[..idx].trim();
            let value = &trimmed[idx + 1..].trim();

            // Determine style based on key content
            let style = if key.to_uppercase().contains("BREAKING") {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            } else if key.to_lowercase().contains("co-authored")
                || key.to_lowercase().contains("signed-off")
            {
                Style::default().fg(Color::Green)
            } else if key.to_lowercase().contains("review") || key.to_lowercase().contains("acked")
            {
                Style::default().fg(Color::Blue)
            } else if key.to_lowercase().contains("fix") || key.to_lowercase().contains("close") {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Magenta)
            };

            return Line::from(vec![
                Span::styled(format!("{}:", key), style),
                Span::styled(format!(" {}", value), Style::default().fg(Color::DarkGray)),
            ]);
        }

        // URL
        if trimmed.starts_with("https://") || trimmed.starts_with("http://") {
            return Line::from(vec![Span::styled(
                trimmed,
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::UNDERLINED),
            )]);
        }

        // Default
        Line::from(Span::styled(trimmed, Style::default().fg(Color::DarkGray)))
    }

    fn extract_digits(s: &str) -> (&str, &str) {
        let idx = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
        (&s[..idx], &s[idx..])
    }

    fn extract_username(s: &str) -> (&str, &str) {
        let idx = s
            .find(|c: char| !c.is_alphanumeric() && c != '-' && c != '_' && c != '.')
            .unwrap_or(s.len());
        (&s[..idx], &s[idx..])
    }
}
