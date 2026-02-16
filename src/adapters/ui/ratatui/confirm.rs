use std::io;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use crate::ports::ui::UiError;

pub struct ConfirmRenderer;

impl ConfirmRenderer {
    /// Render a confirmation dialog
    pub fn render(
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
        message: &str,
        selected: bool,
    ) -> Result<(), UiError> {
        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints([
                        Constraint::Min(0),    // Spacer
                        Constraint::Length(7), // Dialog
                        Constraint::Min(0),    // Spacer
                    ])
                    .split(f.area());

                // Center the dialog
                let dialog_area = chunks[1];
                let dialog_block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan))
                    .title(Span::styled(
                        " Confirmation ",
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ));

                let content_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([
                        Constraint::Length(2), // Message
                        Constraint::Length(1), // Spacer
                        Constraint::Length(1), // Buttons
                    ])
                    .split(dialog_block.inner(dialog_area));

                f.render_widget(dialog_block, dialog_area);

                // Message
                let message_widget = Paragraph::new(message)
                    .style(Style::default().fg(Color::White))
                    .alignment(Alignment::Center)
                    .wrap(ratatui::widgets::Wrap { trim: true });
                f.render_widget(message_widget, content_chunks[0]);

                // Buttons
                let yes_style = if selected {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Green)
                };

                let no_style = if !selected {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Red)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Red)
                };

                let buttons = vec![Line::from(vec![
                    Span::styled("  [ Yes ]  ", yes_style),
                    Span::raw("  "),
                    Span::styled("  [ No ]  ", no_style),
                ])];

                let buttons_widget = Paragraph::new(buttons).alignment(Alignment::Center);
                f.render_widget(buttons_widget, content_chunks[2]);
            })
            .map(|_| ())
            .map_err(|e| UiError(e.to_string()))
    }
}
