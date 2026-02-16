#![cfg(feature = "tui")]
//! Ratatui UI adapter — rich TUI implementation of the Ui port.
//!
//! Provides a modern, interactive terminal UI with:
//! - Styled input fields with visual feedback
//! - Real-time character counting
//! - Syntax highlighting for commit messages
//! - Beautiful preview rendering
//! - Keyboard navigation and shortcuts
//! - Selection menus for predefined options
//!
//! Swappable with TerminalUI — just change one line in cli.rs.

mod confirm;
mod preview;
mod prompt;
mod select;
mod syntax;
mod terminal_setup;

use crate::ports::ui::{Ui, UiError};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};

pub use confirm::ConfirmRenderer;
pub use preview::PreviewRenderer;
pub use prompt::PromptRenderer;
pub use select::{SelectOption, SelectRenderer};
pub use terminal_setup::TerminalManager;

#[derive(Copy, Clone)]
pub struct RatatuiUI;

impl Ui for RatatuiUI {
    fn prompt(&self, label: &str) -> Result<String, UiError> {
        let mut terminal = TerminalManager::setup()?;
        let mut input = String::new();
        let mut cursor_pos = 0;

        // Determine max length from label hints
        let max_length = if label.contains("description") {
            Some(72)
        } else {
            None
        };

        loop {
            PromptRenderer::render(&mut terminal, label, &input, cursor_pos, max_length)?;

            if let Event::Key(key) = event::read().map_err(|e| UiError(e.to_string()))? {
                match key.code {
                    KeyCode::Enter => break,
                    KeyCode::Esc => {
                        TerminalManager::restore(&mut terminal)?;
                        return Err(UiError("Input cancelled".to_string()));
                    }
                    KeyCode::Char(c) => {
                        // Handle Ctrl+C
                        if key.modifiers.contains(KeyModifiers::CONTROL) && c == 'c' {
                            TerminalManager::restore(&mut terminal)?;
                            return Err(UiError("Input cancelled".to_string()));
                        }

                        input.insert(cursor_pos, c);
                        cursor_pos += 1;
                    }
                    KeyCode::Backspace => {
                        if cursor_pos > 0 {
                            cursor_pos -= 1;
                            input.remove(cursor_pos);
                        }
                    }
                    KeyCode::Delete => {
                        if cursor_pos < input.len() {
                            input.remove(cursor_pos);
                        }
                    }
                    KeyCode::Left => {
                        if cursor_pos > 0 {
                            cursor_pos -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if cursor_pos < input.len() {
                            cursor_pos += 1;
                        }
                    }
                    KeyCode::Home => cursor_pos = 0,
                    KeyCode::End => cursor_pos = input.len(),
                    _ => {}
                }
            }
        }

        TerminalManager::restore(&mut terminal)?;
        Ok(input.trim().to_string())
    }

    fn show_preview(&self, content: &str) {
        let mut terminal = match TerminalManager::setup() {
            Ok(t) => t,
            Err(_) => {
                // Fallback to simple print
                println!("\n=== Preview ===\n");
                println!("{}", content);
                println!();
                return;
            }
        };

        let _ = PreviewRenderer::render(&mut terminal, content);

        // Wait for any key
        let _ = event::read();

        let _ = TerminalManager::restore(&mut terminal);
    }

    fn confirm(&self, msg: &str) -> Result<bool, UiError> {
        let mut terminal = TerminalManager::setup()?;
        let mut selected = true; // Default to "Yes"

        loop {
            ConfirmRenderer::render(&mut terminal, msg, selected)?;

            if let Event::Key(key) = event::read().map_err(|e| UiError(e.to_string()))? {
                match key.code {
                    KeyCode::Enter => {
                        TerminalManager::restore(&mut terminal)?;
                        return Ok(selected);
                    }
                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        TerminalManager::restore(&mut terminal)?;
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        TerminalManager::restore(&mut terminal)?;
                        return Ok(false);
                    }
                    KeyCode::Left | KeyCode::Right | KeyCode::Tab => {
                        selected = !selected;
                    }
                    KeyCode::Esc => {
                        TerminalManager::restore(&mut terminal)?;
                        return Ok(false);
                    }
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        TerminalManager::restore(&mut terminal)?;
                        return Ok(false);
                    }
                    _ => {}
                }
            }
        }
    }

    fn select(
        &self,
        title: &str,
        options: Vec<(String, String, String)>,
    ) -> Result<String, UiError> {
        let mut terminal = TerminalManager::setup()?;

        let select_options: Vec<SelectOption> = options
            .into_iter()
            .map(|(value, label, description)| SelectOption {
                value,
                label,
                description,
            })
            .collect();

        let result = SelectRenderer::select(&mut terminal, title, select_options);

        TerminalManager::restore(&mut terminal)?;
        result
    }

    fn println(&self, msg: &str) {
        // with this user will not see what's printed until they exist TUI, which is not a good UX
        println!("{}", msg);
    }
}
