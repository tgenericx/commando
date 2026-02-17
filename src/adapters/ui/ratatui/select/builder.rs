//! Fluent builder for selection UI

use super::options::SelectOption;
use super::state::Selector;
use crate::ports::ui::UiError;
use ratatui::Terminal;

/// Configuration builder for the selection UI
#[derive(Clone)]
pub struct SelectBuilder<T> {
    pub(crate) title: String,
    pub(crate) options: Vec<SelectOption<T>>,
    pub(crate) grid_cols: usize,
    pub(crate) show_preview: bool,
    pub(crate) show_details: bool,
    pub(crate) vim_bindings: bool,
    pub(crate) wrap_navigation: bool,
    pub(crate) quick_jump_numbers: bool,
    pub(crate) help_text: Option<String>,
}

impl<T: Clone> SelectBuilder<T> {
    // Added Clone bound here
    /// Create a new selector with the given title
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            options: Vec::new(),
            grid_cols: 1, // Default to list view
            show_preview: false,
            show_details: true,
            vim_bindings: true,
            wrap_navigation: true,
            quick_jump_numbers: true,
            help_text: None,
        }
    }

    /// Set the options to choose from
    pub fn options(mut self, options: Vec<SelectOption<T>>) -> Self {
        self.options = options;
        self
    }

    /// Set number of grid columns (1 = list view)
    pub fn grid(mut self, cols: usize) -> Self {
        assert!(cols > 0, "grid columns must be positive");
        self.grid_cols = cols;
        self
    }

    /// Show a preview panel for the selected option
    pub fn with_preview(mut self) -> Self {
        self.show_preview = true;
        self
    }

    /// Show detailed information in the preview
    pub fn with_details(mut self) -> Self {
        self.show_details = true;
        self
    }

    /// Enable/disable vim-style navigation (h/j/k/l)
    pub fn vim_bindings(mut self, enabled: bool) -> Self {
        self.vim_bindings = enabled;
        self
    }

    /// Enable/disable wrap-around navigation
    pub fn wrap_navigation(mut self, enabled: bool) -> Self {
        self.wrap_navigation = enabled;
        self
    }

    /// Enable/disable quick jump with number keys (1-9)
    pub fn quick_jump_numbers(mut self, enabled: bool) -> Self {
        self.quick_jump_numbers = enabled;
        self
    }

    /// Set custom help text
    pub fn help_text(mut self, text: impl Into<String>) -> Self {
        self.help_text = Some(text.into());
        self
    }

    /// Run the selection UI and return the selected value
    pub fn run<B: ratatui::backend::Backend>(
        self,
        terminal: &mut Terminal<B>,
    ) -> Result<T, UiError> {
        if self.options.is_empty() {
            return Err(UiError("No options provided".into()));
        }

        let mut selector = Selector::new(self);
        selector.run(terminal)
    }
}
