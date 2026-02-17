//! Preview panel renderer

use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use super::super::options::SelectOption;

pub struct PreviewRenderer;

impl PreviewRenderer {
    pub fn render<T>(
        f: &mut Frame,
        area: Rect,
        option: &SelectOption<T>,
        show_details: bool,
        scroll: u16,
    ) {
        let mut preview_lines = Vec::new();

        // Description
        preview_lines.push(Line::from(vec![
            Span::styled("📝 ", Style::default().fg(Color::Cyan)),
            Span::styled("Description: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(&option.description),
        ]));

        // Details
        if show_details {
            if let Some(details) = &option.details {
                preview_lines.push(Line::from(""));
                preview_lines.push(Line::from(vec![
                    Span::styled("🔍 ", Style::default().fg(Color::Yellow)),
                    Span::styled("Details:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                ]));
                
                for line in details.lines() {
                    preview_lines.push(Line::from(Span::raw(format!("  {}", line))));
                }
            } else {
                preview_lines.push(Line::from(""));
                preview_lines.push(Line::from(vec![
                    Span::styled("ℹ️ ", Style::default().fg(Color::Blue)),
                    Span::styled("No additional details", Style::default().fg(Color::Gray)),
                ]));
            }
        }

        let preview = Paragraph::new(preview_lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Green))
                    .title(format!(" 📋 {} ", option.label))
                    .title_alignment(Alignment::Center),
            )
            .wrap(Wrap { trim: true })
            .scroll((scroll, 0));

        f.render_widget(preview, area);
    }
}
