pub trait StagingChecker {
    type Error;

    fn has_staged_changes(&self) -> Result<bool, Self::Error>;
}
