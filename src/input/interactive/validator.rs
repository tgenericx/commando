use crate::compiler::compile::compile;

#[derive(Debug, Default)]
pub struct InteractiveValidator;

impl InteractiveValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_type(&self, input: &str) -> bool {
        let test_msg = format!("{}: test", input);
        compile(&test_msg).is_ok()
    }

    pub fn validate_scope(&self, input: &str) -> bool {
        if input.is_empty() {
            return true;
        }
        let test_msg = format!("feat({}): test", input);
        compile(&test_msg).is_ok()
    }

    pub fn validate_description(&self, input: &str) -> bool {
        !input.is_empty() && input.len() <= 72
    }
}
