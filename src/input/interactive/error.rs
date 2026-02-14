use crate::domain::DomainError;
use crate::ports::ui::UiError;

#[derive(Debug)]
pub enum InteractiveError {
    Domain(DomainError),
    Ui(UiError),
}

impl std::fmt::Display for InteractiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InteractiveError::Domain(e) => write!(f, "{}", e),
            InteractiveError::Ui(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for InteractiveError {}

impl From<DomainError> for InteractiveError {
    fn from(e: DomainError) -> Self {
        Self::Domain(e)
    }
}

impl From<UiError> for InteractiveError {
    fn from(e: UiError) -> Self {
        Self::Ui(e)
    }
}
