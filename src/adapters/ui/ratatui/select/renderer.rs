//! Main renderer orchestrator

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use super::builder::SelectBuilder;
use super::components::{GridRenderer, ListRenderer, PreviewRenderer};

pub struct SelectRenderer<'a, T> {
    config: &'a SelectBuilder<T>,
}

impl<'a, T> SelectRenderer<'a, T> {
    pub fn new(config: &'a SelectBuilder<T>) -> Self {
        Self { config }
    }

    pub fn draw(&self, f: &mut Frame, selected: usize, preview_scroll: u16) {
        let area = f.area();

        // Main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(self.layout_constraints())
            .split(area);

        // Title
        self.render_title(f, chunks[0]);

        // Content (grid/list + optional preview)
        if self.config.show_preview {
            let content_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[1]);

            self.render_content(f, content_chunks[0], selected);
            self.render_preview(f, content_chunks[1], selected, preview_scroll);
        } else {
            self.render_content(f, chunks[1], selected);
        }

        // Help
        self.render_help(f, chunks[2]);
    }

    fn layout_constraints(&self) -> Vec<Constraint> {
        vec![
            Constraint::Length(3), // Title
            Constraint::Min(10),   // Content
            Constraint::Length(6), // Help
        ]
    }

    fn render_title(&self, f: &mut Frame, area: Rect) {
        // Fixed: Use as_str() to get &str instead of &String
        let title = Paragraph::new(self.config.title.as_str())
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(title, area);
    }

    fn render_content(&self, f: &mut Frame, area: Rect, selected: usize) {
        if self.config.grid_cols == 1 {
            ListRenderer::render(f, area, &self.config.options, selected);
        } else {
            GridRenderer::render(
                f,
                area,
                &self.config.options,
                selected,
                self.config.grid_cols,
            );
        }
    }

    fn render_preview(&self, f: &mut Frame, area: Rect, selected: usize, scroll: u16) {
        let option = &self.config.options[selected];
        PreviewRenderer::render(f, area, option, self.config.show_details, scroll);
    }

    fn render_help(&self, f: &mut Frame, area: Rect) {
        let help_text = if let Some(custom) = &self.config.help_text {
            vec![Line::from(custom.as_str())]
        } else {
            self.default_help_text()
        };

        let help = Paragraph::new(help_text)
            .alignment(ratatui::layout::Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title(" ⌨️ Help "));

        f.render_widget(help, area);
    }

    fn default_help_text(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        // Navigation line
        let mut nav_spans = vec![
            Span::styled(
                "←/→",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" move  "),
        ];

        if self.config.vim_bindings {
            nav_spans.extend(vec![
                Span::styled(
                    "h/l",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("  "),
            ]);
        }

        lines.push(Line::from(nav_spans));

        // Actions line
        lines.push(Line::from(vec![
            Span::styled(
                "Enter",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" select  "),
            Span::styled(
                "Esc/Ctrl+C",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" cancel  "),
            Span::styled(
                "u/d",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" scroll"),
        ]));

        lines
    }
}
