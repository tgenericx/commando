//! State machine for preview UI

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::Terminal;

use crate::ports::ui::UiError;

use super::action::Action;
use super::renderer::PreviewRenderer;

pub struct PreviewState {
    scroll: u16,
    content_lines: Vec<String>,
    total_lines: usize,
    config: PreviewConfig,
}

pub struct PreviewConfig {
    pub title: String,
    pub vim_bindings: bool,
    pub wrap: bool,
    pub syntax_highlighting: bool,
    pub show_scrollbar: bool,
    pub page_size: u16,
}

impl PreviewState {
    pub fn new(content: &str, config: PreviewConfig) -> Self {
        let content_lines: Vec<String> = content.lines().map(String::from).collect();
        let total_lines = content_lines.len();

        Self {
            scroll: 0,
            content_lines,
            total_lines,
            config,
        }
    }

    pub fn run<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> Result<(), UiError> {
        loop {
            self.render(terminal)?;

            if let Event::Key(key) = event::read()? {
                if let Some(action) = self.handle_key(key) {
                    match action {
                        Action::Close => return Ok(()),
                        Action::ScrollUp(amount) => {
                            self.scroll = self.scroll.saturating_sub(amount);
                        }
                        Action::ScrollDown(amount) => {
                            let max_scroll = self.total_lines.saturating_sub(1) as u16;
                            self.scroll = self.scroll.saturating_add(amount).min(max_scroll);
                        }
                        Action::PageUp => {
                            self.scroll = self.scroll.saturating_sub(self.config.page_size);
                        }
                        Action::PageDown => {
                            let max_scroll = self.total_lines.saturating_sub(1) as u16;
                            self.scroll = self
                                .scroll
                                .saturating_add(self.config.page_size)
                                .min(max_scroll);
                        }
                        Action::Home => {
                            self.scroll = 0;
                        }
                        Action::End => {
                            self.scroll = self.total_lines.saturating_sub(1) as u16;
                        }
                    }
                }
            }
        }
    }

    fn handle_key(&self, key: crossterm::event::KeyEvent) -> Option<Action> {
        match key.code {
            // Close
            KeyCode::Esc | KeyCode::Enter => Some(Action::Close),
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                Some(Action::Close)
            }
            KeyCode::Char('q') => Some(Action::Close),

            // Scrolling
            KeyCode::Up => Some(Action::ScrollUp(1)),
            KeyCode::Down => Some(Action::ScrollDown(1)),
            KeyCode::PageUp => Some(Action::PageUp),
            KeyCode::PageDown => Some(Action::PageDown),

            // Vim bindings
            KeyCode::Char('k') if self.config.vim_bindings => Some(Action::ScrollUp(1)),
            KeyCode::Char('j') if self.config.vim_bindings => Some(Action::ScrollDown(1)),
            KeyCode::Char('u') if self.config.vim_bindings => Some(Action::PageUp),
            KeyCode::Char('d') if self.config.vim_bindings => Some(Action::PageDown),

            // Home/End
            KeyCode::Home => Some(Action::Home),
            KeyCode::End => Some(Action::End),

            _ => None,
        }
    }

    fn render<B: ratatui::backend::Backend>(
        &self,
        terminal: &mut Terminal<B>,
    ) -> Result<(), UiError> {
        terminal
            .draw(|f| {
                let renderer = PreviewRenderer::new(&self.config);
                renderer.draw(f, &self.content_lines, self.scroll, self.total_lines);
            })
            .map_err(|e| UiError(e.to_string()))?;
        Ok(())
    }
}
