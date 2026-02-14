#[derive(Debug, Clone, PartialEq)]
pub enum GitError {
    ExecutionFailed(String),
    ShaLookupFailed(String),
}
