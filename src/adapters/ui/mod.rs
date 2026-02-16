#[cfg(feature = "tui")]
mod ratatui;

mod terminal;

pub use terminal::TerminalUI;

#[cfg(feature = "tui")]
pub use ratatui::RatatuiUI;
