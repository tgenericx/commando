mod commit;

pub use commit::CommitExecutor;

#[derive(Debug, Default)]
pub struct CommitResult {
    pub sha: String,
    pub summary: String,
}
