//! Fluent builder for text prompts

use super::state::PromptState;
use crate::ports::ui::UiError;
use ratatui::Terminal;

/// Type alias for validator function
pub type ValidatorFn<'a> = Box<dyn Fn(&str) -> Result<(), String> + 'a>;

/// Configuration for a text prompt
pub struct PromptBuilder<'a> {
    pub(crate) message: &'a str,
    pub(crate) initial_value: String,
    pub(crate) placeholder: Option<&'a str>,
    pub(crate) validator: Option<ValidatorFn<'a>>,
    pub(crate) max_length: Option<usize>,
    pub(crate) secret: bool,
    pub(crate) help_text: Option<String>,
    pub(crate) allow_cancel: bool,
}

impl<'a> PromptBuilder<'a> {
    /// Create a new prompt with the given message
    pub fn new(message: &'a str) -> Self {
        Self {
            message,
            initial_value: String::new(),
            placeholder: None,
            validator: None,
            max_length: None,
            secret: false,
            help_text: None,
            allow_cancel: true,
        }
    }

    /// Set placeholder text when input is empty
    pub fn placeholder(mut self, text: &'a str) -> Self {
        self.placeholder = Some(text);
        self
    }

    /// Set maximum input length
    pub fn max_length(mut self, max: usize) -> Self {
        self.max_length = Some(max);
        self
    }

    /// Set custom help text
    pub fn help_text(mut self, text: impl Into<String>) -> Self {
        self.help_text = Some(text.into());
        self
    }

    /// Allow/disallow cancellation with Esc/Ctrl+C
    pub fn allow_cancel(mut self, yes: bool) -> Self {
        self.allow_cancel = yes;
        self
    }

    /// Run the prompt and return the input
    pub fn run<B: ratatui::backend::Backend>(
        self,
        terminal: &mut Terminal<B>,
    ) -> Result<String, UiError> {
        let mut state = PromptState::new(self);
        state.run(terminal)
    }
}
