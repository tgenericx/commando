/// UI port — abstract interface for all terminal interaction.
///
/// Both TerminalUI (plain stdin/stdout) and RatatuiUI implement this.
/// InteractiveSource and AppController depend only on this trait,
/// never on concrete types.
#[derive(Debug)]
pub struct UiError(pub String);

impl std::fmt::Display for UiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UI error: {}", self.0)
    }
}

impl std::error::Error for UiError {}

impl From<std::io::Error> for UiError {
    fn from(e: std::io::Error) -> Self {
        UiError(e.to_string())
    }
}

pub trait Ui {
    /// Prompt the user with a label, return trimmed input.
    fn prompt(&self, label: &str) -> Result<String, UiError>;

    /// Show the commit preview.
    fn show_preview(&self, content: &str);

    /// Ask a yes/no question. Returns true for y/yes.
    fn confirm(&self, msg: &str) -> Result<bool, UiError>;

    /// Print a line (with newline).
    fn println(&self, msg: &str);
}
