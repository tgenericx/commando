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

    /// Show a preview of content (e.g., final commit message)
    fn show_preview(&self, content: &str);

    /// Ask for yes/no confirmation
    fn confirm(&self, msg: &str) -> Result<bool, UiError>;

    /// Display a selection menu and return the selected value
    ///
    /// # Arguments
    /// * `title` - The title/prompt for the selection
    /// * `options` - A vector of tuples (value, label, description)
    ///   - value: The string value to return when selected
    ///   - label: The display label for the option
    ///   - description: A brief description of what the option means
    ///
    /// # Returns
    /// The value string of the selected option
    fn select(
        &self,
        title: &str,
        options: Vec<(String, String, String)>,
    ) -> Result<String, UiError>;

    /// Print a message to the user
    fn println(&self, msg: &str);
}
