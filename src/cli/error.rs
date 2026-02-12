use std::io;

#[derive(Debug)]
pub enum CliError {
    NoStagedChanges,
    GitError(String),
    Editor(String),
    Io(io::Error),
    EmptyMessage,
    CompileFailed,
    CompileError(String),
    UserCancelled,
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::NoStagedChanges => write!(f, "No staged changes found"),
            CliError::GitError(e) => write!(f, "Git error: {}", e),
            CliError::Editor(e) => write!(f, "Editor error: {}", e),
            CliError::Io(e) => write!(f, "File error: {}", e),
            CliError::EmptyMessage => write!(f, "Empty commit message"),
            CliError::CompileFailed => write!(f, "Commit message compilation failed"),
            CliError::CompileError(e) => write!(f, "Compile error: {}", e),
            CliError::UserCancelled => write!(f, "Cancelled by user"),
        }
    }
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> Self {
        CliError::Io(err)
    }
}
