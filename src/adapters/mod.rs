//! Adapters — concrete implementations of ports.
//!
//! All imports in this crate should go through these re-exports.
//! Nothing outside adapters/ should import adapter internals directly.
mod git;
pub mod ui;

pub use git::GitCommitExecutor;
pub use git::GitStagingChecker;
pub use ui::TerminalUI;
