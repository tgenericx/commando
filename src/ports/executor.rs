use crate::domain::error::DomainError;

pub struct CommitResult {
    pub sha: String,
    pub summary: String,
}

pub trait CommitExecutor {
    fn execute(&self, message: &str) -> Result<CommitResult, DomainError>;
}

pub trait DryRunner {
    fn dry_run(&self, message: &str) -> Result<(), DomainError>;
}
