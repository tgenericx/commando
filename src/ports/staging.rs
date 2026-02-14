use crate::domain::error::DomainError;

pub trait StagingChecker {
    fn has_staged_changes(&self) -> Result<bool, DomainError>;
}
