//! State machine for selection UI

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::Terminal;

use crate::adapters::ui::ratatui::SelectOption;
use crate::ports::ui::UiError;

use super::action::Action;
use super::builder::SelectBuilder;
use super::geometry::GridPosition;
use super::renderer::SelectRenderer;

pub struct Selector<T> {
    pub(crate) config: SelectBuilder<T>,
    pub(crate) selected: usize,
    pub(crate) preview_scroll: u16,
    pub(crate) options: Vec<SelectOption<T>>,
}

impl<T: Clone> Selector<T> {
    pub fn new(config: SelectBuilder<T>) -> Self {
        let options = config.options.clone();
        Self {
            selected: 0,
            preview_scroll: 0,
            options,
            config,
        }
    }

    pub fn run<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> Result<T, UiError> {
        loop {
            self.render(terminal)?;

            if let Event::Key(key) = event::read()? {
                if let Some(action) = self.handle_key(key) {
                    match action {
                        Action::Select => return Ok(self.options[self.selected].value.clone()),
                        Action::Cancel => return Err(UiError("Selection cancelled".into())), // Fixed: removed .Other
                        Action::Move(new_idx) => {
                            self.selected = new_idx;
                            self.preview_scroll = 0;
                        }
                        Action::ScrollPreview(delta) => {
                            self.preview_scroll = self.preview_scroll.saturating_add_signed(delta);
                        }
                    }
                }
            }
        }
    }

    fn handle_key(&self, key: crossterm::event::KeyEvent) -> Option<Action> {
        let pos = GridPosition::new(self.selected, self.options.len(), self.config.grid_cols);

        match key.code {
            // Selection/Cancellation
            KeyCode::Enter => Some(Action::Select),
            KeyCode::Esc => Some(Action::Cancel),
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                Some(Action::Cancel)
            }

            // Navigation
            KeyCode::Up => Some(Action::Move(pos.move_up()?)),
            KeyCode::Down => Some(Action::Move(pos.move_down()?)),
            KeyCode::Left => Some(Action::Move(pos.move_left()?)),
            KeyCode::Right => Some(Action::Move(pos.move_right()?)),

            // Vim bindings
            KeyCode::Char('k') if self.config.vim_bindings => Some(Action::Move(pos.move_up()?)),
            KeyCode::Char('j') if self.config.vim_bindings => Some(Action::Move(pos.move_down()?)),
            KeyCode::Char('h') if self.config.vim_bindings => Some(Action::Move(pos.move_left()?)),
            KeyCode::Char('l') if self.config.vim_bindings => Some(Action::Move(pos.move_right()?)),

            // Home/End
            KeyCode::Home => Some(Action::Move(pos.home())),
            KeyCode::End => Some(Action::Move(pos.end())),

            // Preview scrolling
            KeyCode::Char('u') => Some(Action::ScrollPreview(-1)),
            KeyCode::Char('d') => Some(Action::ScrollPreview(1)),

            // Quick jump numbers
            KeyCode::Char(c) if self.config.quick_jump_numbers && c.is_ascii_digit() => {
                if let Some(digit) = c.to_digit(10) {
                    pos.jump_to_number(digit).map(Action::Move)
                } else {
                    None
                }
            }

            _ => None,
        }
    }

    fn render<B: ratatui::backend::Backend>(
        &self,
        terminal: &mut Terminal<B>,
    ) -> Result<(), UiError> {
        terminal
            .draw(|f| {
                let renderer = SelectRenderer::new(&self.config);
                renderer.draw(f, self.selected, self.preview_scroll);
            })
            .map_err(|e| UiError(e.to_string()))?;
        Ok(())
    }
}
