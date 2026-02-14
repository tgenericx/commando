//! Adapters module - Concrete implementations of ports
//!
//! This module contains the actual implementations of the ports defined in the ports module.
//! Currently, we have Git-based implementations for all ports.

mod git;

pub use git::GitCommitExecutor;
pub use git::GitStagingChecker;
