//! Selection UI with grid/list view and live preview
//!
//! # Example
//! ```
//! use your_crate::adapters::ui::ratatui::select::{SelectBuilder, SelectOption};
//!
//! let options = vec![
//!     SelectOption::new("rust", "🦀 Rust", "Systems programming")
//!         .with_details("Memory safe • Zero-cost abstractions"),
//!     SelectOption::new("python", "🐍 Python", "High-level language")
//!         .with_details("Great for data science • Easy to learn"),
//! ];
//!
//! let selected = SelectBuilder::new("Choose language")
//!     .options(options)
//!     .grid(2)
//!     .with_preview()
//!     .vim_bindings(true)
//!     .run(&mut terminal)?;
//! ```

mod action;
mod builder;
mod geometry;
mod options;
mod renderer;
mod state;
mod components;

pub use builder::SelectBuilder;
pub use options::SelectOption;
