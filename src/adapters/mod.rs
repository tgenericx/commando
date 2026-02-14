//! Adapters module - Concrete implementations of ports
//!
//! This module contains the actual implementations of the ports defined in the ports module.
//! Currently, we have Git-based implementations for all ports.

mod git_executor;
mod git_staging;

pub use git_executor::GitCommitExecutor;
pub use git_staging::GitStagingChecker;
