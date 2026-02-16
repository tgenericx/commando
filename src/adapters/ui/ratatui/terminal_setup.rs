use crate::ports::ui::UiError;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{self, stdout};

pub struct TerminalManager;

impl TerminalManager {
    /// Initialize the terminal for TUI mode
    pub fn setup() -> Result<Terminal<CrosstermBackend<io::Stdout>>, UiError> {
        enable_raw_mode().map_err(|e| UiError(e.to_string()))?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen).map_err(|e| UiError(e.to_string()))?;
        let backend = CrosstermBackend::new(stdout);
        Terminal::new(backend).map_err(|e| UiError(e.to_string()))
    }

    /// Clean up terminal state
    pub fn restore(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), UiError> {
        disable_raw_mode().map_err(|e| UiError(e.to_string()))?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)
            .map_err(|e| UiError(e.to_string()))?;
        terminal.show_cursor().map_err(|e| UiError(e.to_string()))?;
        Ok(())
    }
}
