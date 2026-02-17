//! Prompt state machine

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::Terminal;

use super::builder::PromptBuilder;
use super::renderer::PromptRenderer;
use super::validation::{ValidationState, Validator};
use crate::ports::ui::UiError;

pub struct PromptState<'a> {
    config: PromptBuilder<'a>,
    input: String,
    cursor_pos: usize,
    validation: ValidationState,
}

impl<'a> PromptState<'a> {
    pub fn new(config: PromptBuilder<'a>) -> Self {
        let input = config.initial_value.clone();
        let cursor_pos = input.chars().count();

        Self {
            config,
            input,
            cursor_pos,
            validation: ValidationState::Valid,
        }
    }

    pub fn run<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> Result<String, UiError> {
        loop {
            // Update validation
            self.validate();

            // Render
            terminal
                .draw(|f| {
                    PromptRenderer::render(
                        f,
                        &self.config,
                        &self.input,
                        self.cursor_pos,
                        &self.validation,
                    );
                })
                .map_err(|e| UiError(e.to_string()))?;

            // Handle input
            if let Event::Key(key) = event::read()? {
                match self.handle_key(key) {
                    PromptAction::Submit => {
                        if self.is_valid() {
                            return Ok(self.input.clone());
                        }
                        // If invalid, stay in loop
                    }
                    PromptAction::Cancel => {
                        if self.config.allow_cancel {
                            return Err(UiError("Prompt cancelled".into()));
                        }
                    }
                    PromptAction::Continue => continue,
                }
            }
        }
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> PromptAction {
        match key.code {
            // Cancel - check this BEFORE the general Char arm
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                PromptAction::Cancel
            }

            // Text input
            KeyCode::Char(c) => {
                if Validator::is_ascii_printable(c) {
                    self.insert_char(c);
                }
                PromptAction::Continue
            }

            // Backspace
            KeyCode::Backspace => {
                self.delete_char();
                PromptAction::Continue
            }

            // Delete (forward delete)
            KeyCode::Delete => {
                self.delete_forward();
                PromptAction::Continue
            }

            // Cursor movement
            KeyCode::Left => {
                self.move_cursor_left();
                PromptAction::Continue
            }
            KeyCode::Right => {
                self.move_cursor_right();
                PromptAction::Continue
            }
            KeyCode::Home => {
                self.cursor_pos = 0;
                PromptAction::Continue
            }
            KeyCode::End => {
                self.cursor_pos = self.input.chars().count();
                PromptAction::Continue
            }

            // Submit
            KeyCode::Enter => PromptAction::Submit,

            // Cancel
            KeyCode::Esc => PromptAction::Cancel,

            _ => PromptAction::Continue,
        }
    }

    fn insert_char(&mut self, c: char) {
        // Collapsed if statement
        if let Some(max) = self.config.max_length
            && self.input.chars().count() >= max
        {
            return;
        }

        let mut chars: Vec<char> = self.input.chars().collect();
        chars.insert(self.cursor_pos, c);
        self.input = chars.into_iter().collect();
        self.cursor_pos += 1;
    }

    fn delete_char(&mut self) {
        if self.cursor_pos > 0 {
            let mut chars: Vec<char> = self.input.chars().collect();
            chars.remove(self.cursor_pos - 1);
            self.input = chars.into_iter().collect();
            self.cursor_pos -= 1;
        }
    }

    fn delete_forward(&mut self) {
        if self.cursor_pos < self.input.chars().count() {
            let mut chars: Vec<char> = self.input.chars().collect();
            chars.remove(self.cursor_pos);
            self.input = chars.into_iter().collect();
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_pos < self.input.chars().count() {
            self.cursor_pos += 1;
        }
    }

    fn validate(&mut self) {
        // Length validation
        self.validation = Validator::validate_length(&self.input, self.config.max_length);

        // Custom validation (if valid so far) - collapsed if statement
        if let ValidationState::Valid = self.validation
            && let Some(validator) = &self.config.validator
        {
            self.validation = Validator::apply_custom(&self.input, Some(validator));
        }
    }

    fn is_valid(&self) -> bool {
        matches!(self.validation, ValidationState::Valid)
    }
}

enum PromptAction {
    Submit,
    Cancel,
    Continue,
}
