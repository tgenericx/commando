//! Git adapter implementations
//!
//! This module contains Git-based implementations of the ports.

mod error;
mod executor;
mod staging;

pub use error::{GitError, Result};
pub use executor::GitCommitExecutor;
pub use staging::GitStagingChecker;
