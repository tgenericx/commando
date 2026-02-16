use crate::ports::ui::UiError;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use std::io;

pub struct PromptRenderer;

impl PromptRenderer {
    /// Render a prompt with real-time input
    pub fn render(
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
        label: &str,
        input: &str,
        cursor_pos: usize,
        max_length: Option<usize>,
    ) -> Result<(), UiError> {
        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints([
                        Constraint::Length(3), // Title
                        Constraint::Length(3), // Input field
                        Constraint::Length(3), // Help text
                        Constraint::Min(0),    // Spacer
                    ])
                    .split(f.area());

                // Title
                let title = Paragraph::new(label)
                    .style(
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )
                    .alignment(Alignment::Left)
                    .block(Block::default().borders(Borders::NONE));
                f.render_widget(title, chunks[0]);

                // Input field with character count
                let char_count = input.chars().count();
                let count_color = if let Some(max) = max_length {
                    if char_count > max {
                        Color::Red
                    } else if char_count > (max * 80 / 100) {
                        Color::Yellow
                    } else {
                        Color::Green
                    }
                } else {
                    Color::Green
                };

                let count_text = if let Some(max) = max_length {
                    format!(" [{}/{}]", char_count, max)
                } else {
                    format!(" [{}]", char_count)
                };

                let input_block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Gray))
                    .title(Span::styled(
                        count_text,
                        Style::default()
                            .fg(count_color)
                            .add_modifier(Modifier::BOLD),
                    ));

                let input_widget = Paragraph::new(input)
                    .style(Style::default().fg(Color::White))
                    .block(input_block);
                f.render_widget(input_widget, chunks[1]);

                // Calculate cursor position for rendering
                let cursor_x = chunks[1].x + 1 + cursor_pos as u16;
                let cursor_y = chunks[1].y + 1;
                f.set_cursor_position((cursor_x, cursor_y));

                // Help text
                let help = vec![Line::from(vec![
                    Span::styled("Enter", Style::default().fg(Color::Green)),
                    Span::raw(" to confirm | "),
                    Span::styled("Esc", Style::default().fg(Color::Red)),
                    Span::raw(" to cancel"),
                ])];
                let help_widget = Paragraph::new(help)
                    .style(Style::default().fg(Color::DarkGray))
                    .alignment(Alignment::Center);
                f.render_widget(help_widget, chunks[2]);
            })
            .map(|_| ())
            .map_err(|e| UiError(e.to_string()))
    }
}
