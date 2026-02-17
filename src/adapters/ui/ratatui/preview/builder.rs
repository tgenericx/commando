//! Fluent builder for preview UI

use crate::ports::ui::UiError;
use ratatui::Terminal;

use super::state::{PreviewConfig, PreviewState};

/// Configuration builder for the preview UI
#[derive(Clone)]
pub struct PreviewBuilder {
    title: String,
    content: String,
    vim_bindings: bool,
    wrap: bool,
    syntax_highlighting: bool,
    show_scrollbar: bool,
    page_size: u16,
}

impl PreviewBuilder {
    /// Create a new preview builder with the given title
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: String::new(),
            vim_bindings: true,
            wrap: true,
            syntax_highlighting: true,
            show_scrollbar: true,
            page_size: 5,
        }
    }

    /// Set the content to preview
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    /// Enable/disable vim-style navigation (j/k, u/d)
    pub fn vim_bindings(mut self, enabled: bool) -> Self {
        self.vim_bindings = enabled;
        self
    }

    /// Enable/disable text wrapping
    pub fn wrap(mut self, enabled: bool) -> Self {
        self.wrap = enabled;
        self
    }

    /// Enable/disable scrollbar
    pub fn show_scrollbar(mut self, enabled: bool) -> Self {
        self.show_scrollbar = enabled;
        self
    }

    /// Set number of lines to scroll per page
    pub fn page_size(mut self, size: u16) -> Self {
        self.page_size = size;
        self
    }

    /// Show the preview and return when user exits
    pub fn show<B: ratatui::backend::Backend>(
        self,
        terminal: &mut Terminal<B>,
    ) -> Result<(), UiError> {
        if self.content.is_empty() {
            return Err(UiError("No content to preview".into()));
        }

        let config = PreviewConfig {
            title: self.title,
            vim_bindings: self.vim_bindings,
            wrap: self.wrap,
            syntax_highlighting: self.syntax_highlighting,
            show_scrollbar: self.show_scrollbar,
            page_size: self.page_size,
        };

        let mut state = PreviewState::new(&self.content, config);
        state.run(terminal)
    }
}
