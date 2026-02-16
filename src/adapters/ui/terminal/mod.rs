//! Terminal UI adapter — plain stdin/stdout implementation of the Ui port.
//!
//! This is the production UI. RatatuiUI will be a second impl of the same
//! trait. Swapping them requires changing one line in cli.rs.

mod ui_impl;

pub use ui_impl::TerminalUI;
