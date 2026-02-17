//! Scrollable preview UI with configurable options
//!
//! # Example
//! ```
//! use your_crate::adapters::ui::ratatui::preview::PreviewBuilder;
//!
//! PreviewBuilder::new("Commit Message")
//!     .content("feat: add awesome feature\n\nThis is a long description...")
//!     .vim_bindings(true)
//!     .wrap(true)
//!     .show(&mut terminal)?;
//!
//! // With syntax highlighting
//! PreviewBuilder::new("Diff Preview")
//!     .content(diff_content)
//!     .syntax_highlighting(true)
//!     .page_size(10)
//!     .show(&mut terminal)?;
//! ```

mod action;
mod builder;
mod renderer;
mod state;

pub use builder::PreviewBuilder;
