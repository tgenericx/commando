//! Text input prompt with validation and real-time feedback
//!
//! # Example
//! ```
//! use your_crate::adapters::ui::ratatui::prompt::PromptBuilder;
//!
//! let name = PromptBuilder::new("What's your name?")
//!     .placeholder("Enter name...")
//!     .max_length(50)
//!     .validator(|s| {
//!         if s.is_empty() {
//!             Err("Name cannot be empty".into())
//!         } else {
//!             Ok(())
//!         }
//!     })
//!     .run(&mut terminal)?;
//!
//! let password = PromptBuilder::new("Enter password:")
//!     .secret(true)
//!     .validator(|s| {
//!         if s.len() < 8 {
//!             Err("Password too short".into())
//!         } else {
//!             Ok(())
//!         }
//!     })
//!     .run(&mut terminal)?;
//! ```

mod builder;
mod renderer;
mod state;
mod validation;

pub use builder::PromptBuilder;
