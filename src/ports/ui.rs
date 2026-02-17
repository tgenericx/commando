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

/// The Ui port — abstracts all user interaction
pub trait Ui {
    /// Prompt the user for a single line of input
    fn prompt(&self, label: &str) -> Result<String, UiError>;

    /// Generic selection method
    fn select<T: Clone>(
        &self,
        title: &str,
        options: Vec<(T, String, String)>,
    ) -> Result<T, UiError>;

    /// Show a preview of content (e.g., final commit message)
    fn show_preview(&self, content: &str);

    fn println(&self, msg: &str);
}
