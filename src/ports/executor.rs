pub struct CommitResult {
    pub sha: String,
    pub summary: String,
}

pub trait CommitExecutor {
    type Error;

    fn execute(&self, message: &str) -> Result<CommitResult, Self::Error>;
}

pub trait DryRunner {
    type Error;

    fn dry_run(&self, message: &str) -> Result<(), Self::Error>;
}
