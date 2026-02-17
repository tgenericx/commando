//! Prompt renderer

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use super::builder::PromptBuilder;
use super::validation::ValidationState;

pub struct PromptRenderer;

impl PromptRenderer {
    pub fn render(
        f: &mut Frame,
        config: &PromptBuilder,
        input: &str,
        cursor_pos: usize,
        validation: &ValidationState,
    ) {
        let area = f.area();

        // Layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3), // Message
                Constraint::Length(5), // Input field
                Constraint::Length(3), // Validation/Help
                Constraint::Min(0),    // Spacer
            ])
            .split(area);

        // Message
        Self::render_message(f, chunks[0], config.message);

        // Input field
        let input_area = Self::render_input(f, chunks[1], config, input, validation);

        // Validation/Help
        Self::render_footer(f, chunks[2], config, validation);

        // Set cursor position
        if let Some(cursor_x) = Self::calculate_cursor_x(input_area, cursor_pos) {
            f.set_cursor_position((cursor_x, input_area.y + 1));
        }
    }

    fn render_message(f: &mut Frame, area: ratatui::layout::Rect, message: &str) {
        let message_widget = Paragraph::new(message)
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Left);
        f.render_widget(message_widget, area);
    }

    fn render_input(
        f: &mut Frame,
        area: ratatui::layout::Rect,
        config: &PromptBuilder,
        input: &str,
        validation: &ValidationState,
    ) -> ratatui::layout::Rect {
        // Determine border color based on validation
        let border_color = match validation {
            ValidationState::Valid => Color::Green,
            ValidationState::Invalid(_) => Color::Red,
            ValidationState::Warning(_) => Color::Yellow,
        };

        // Character count display
        let char_count = input.chars().count();
        let count_text = if let Some(max) = config.max_length {
            format!(" [{}/{}]", char_count, max)
        } else {
            format!(" [{}]", char_count)
        };

        let count_color = match validation {
            ValidationState::Valid => Color::Green,
            ValidationState::Invalid(_) => Color::Red,
            ValidationState::Warning(_) => Color::Yellow,
        };

        // Input block
        let input_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .title(Span::styled(
                count_text,
                Style::default()
                    .fg(count_color)
                    .add_modifier(Modifier::BOLD),
            ));

        // Display text (masked for secrets)
        let display_text = if config.secret && !input.is_empty() {
            "•".repeat(input.chars().count())
        } else if input.is_empty() {
            config.placeholder.unwrap_or("").to_string()
        } else {
            input.to_string()
        };

        let display_style = if input.is_empty() && config.placeholder.is_some() {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default().fg(Color::White)
        };

        let input_widget = Paragraph::new(display_text)
            .style(display_style)
            .block(input_block);

        f.render_widget(input_widget, area);
        area
    }

    fn render_footer(
        f: &mut Frame,
        area: ratatui::layout::Rect,
        config: &PromptBuilder,
        validation: &ValidationState,
    ) {
        let (validation_text, validation_color) = match validation {
            ValidationState::Invalid(msg) => (msg.as_str(), Color::Red),
            ValidationState::Warning(msg) => (msg.as_str(), Color::Yellow),
            ValidationState::Valid => ("✓ Valid input", Color::Green),
        };

        // Help text (if provided) or default
        let help_text = if let Some(custom) = &config.help_text {
            vec![Line::from(custom.as_str())]
        } else {
            Self::default_help_text(config.allow_cancel)
        };

        // Combine validation and help
        let mut content = vec![
            Line::from(vec![
                Span::styled(
                    "Validation: ",
                    Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(validation_text, Style::default().fg(validation_color)),
            ]),
            Line::from(""),
        ];

        // Extend with help text lines
        content.extend(help_text);

        let footer = Paragraph::new(content)
            .alignment(Alignment::Left)
            .block(Block::default().borders(Borders::NONE));

        f.render_widget(footer, area);
    }

    // Fixed: Add 'static lifetime
    fn default_help_text(allow_cancel: bool) -> Vec<Line<'static>> {
        let mut spans = vec![
            Span::styled(
                "←/→",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" move cursor  "),
            Span::styled(
                "Home/End",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" jump  "),
            Span::styled(
                "Enter",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" confirm"),
        ];

        if allow_cancel {
            spans.extend(vec![
                Span::raw("  "),
                Span::styled(
                    "Esc/Ctrl+C",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" cancel"),
            ]);
        }

        vec![Line::from(spans)]
    }

    fn calculate_cursor_x(input_area: ratatui::layout::Rect, cursor_pos: usize) -> Option<u16> {
        if cursor_pos <= input_area.width.saturating_sub(2) as usize {
            Some(input_area.x + 1 + cursor_pos as u16)
        } else {
            None
        }
    }
}
