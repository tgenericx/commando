use super::syntax::SyntaxHighlighter;
use crate::ports::ui::UiError;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use std::io;

pub struct PreviewRenderer;

impl PreviewRenderer {
    /// Render the commit preview with syntax highlighting
    pub fn render(
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
        content: &str,
    ) -> Result<(), UiError> {
        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints([
                        Constraint::Length(3), // Title
                        Constraint::Min(5),    // Preview content
                        Constraint::Length(3), // Help text
                    ])
                    .split(f.area());

                // Title
                let title = Paragraph::new("Commit Message Preview")
                    .style(
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Cyan)),
                    );
                f.render_widget(title, chunks[0]);

                // Preview content with syntax highlighting
                let highlighted_content = SyntaxHighlighter::highlight_commit_message(content);
                let preview = Paragraph::new(highlighted_content)
                    .style(Style::default())
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Gray)),
                    )
                    .wrap(Wrap { trim: false });
                f.render_widget(preview, chunks[1]);

                // Help text
                let help = vec![Line::from(vec![Span::styled(
                    "Press any key to continue...",
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::ITALIC),
                )])];
                let help_widget = Paragraph::new(help).alignment(Alignment::Center);
                f.render_widget(help_widget, chunks[2]);
            })
            .map(|_| ())
            .map_err(|e| UiError(e.to_string()))
    }
}
