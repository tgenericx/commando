//! Rendering logic for preview

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarState, Wrap},
};

use super::state::PreviewConfig;
use crate::adapters::ui::ratatui::highlight_syntax::CommitHighlighter;

pub struct PreviewRenderer<'a> {
    config: &'a PreviewConfig,
}

impl<'a> PreviewRenderer<'a> {
    pub fn new(config: &'a PreviewConfig) -> Self {
        Self { config }
    }

    pub fn draw(&self, f: &mut Frame, content_lines: &[String], scroll: u16, total_lines: usize) {
        let area = f.area();

        // Main layout - leave space for scrollbar
        let horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(area);

        let content_area = horizontal[0];
        let scrollbar_area = horizontal[1];

        // Title with scroll position
        let title = self.format_title(scroll, total_lines);

        // Prepare content with highlighting
        let content = self.prepare_content(content_lines);

        // Create main paragraph
        let mut paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .title_alignment(Alignment::Center)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .scroll((scroll, 0));

        if self.config.wrap {
            paragraph = paragraph.wrap(Wrap { trim: true });
        }

        // Render main content
        f.render_widget(paragraph, content_area);

        // Render scrollbar if enabled using Ratatui's built-in scrollbar
        if self.config.show_scrollbar && total_lines > 0 {
            let mut scrollbar_state = ScrollbarState::new(total_lines).position(scroll as usize);

            // Create vertical scrollbar
            let scrollbar = Scrollbar::default()
                .orientation(ratatui::widgets::ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None)
                .track_symbol(Some("│"))
                .thumb_symbol("█");

            f.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
        }
    }

    fn format_title(&self, scroll: u16, total_lines: usize) -> String {
        if total_lines > 0 {
            let percent = if total_lines > 0 {
                ((scroll as f32 / (total_lines - 1) as f32) * 100.0) as u8
            } else {
                0
            };
            format!(" {} [{}%] ", self.config.title, percent)
        } else {
            format!(" {} ", self.config.title)
        }
    }

    fn prepare_content(&self, content_lines: &[String]) -> Vec<Line<'static>> {
        if self.config.syntax_highlighting {
            // Convert to owned lines by cloning the spans
            let full_content = content_lines.join("\n");
            let highlighted = CommitHighlighter::highlight(&full_content);

            // Convert to owned lines with 'static lifetime
            highlighted
                .into_iter()
                .map(|line| {
                    let spans: Vec<Span<'static>> = line
                        .spans
                        .into_iter()
                        .map(|span| {
                            Span::styled(
                                span.content.to_string(), // Convert to owned String
                                span.style,
                            )
                        })
                        .collect();
                    Line::from(spans)
                })
                .collect()
        } else {
            // Plain text rendering
            content_lines
                .iter()
                .map(|l| Line::from(Span::raw(l.clone())))
                .collect()
        }
    }
}
