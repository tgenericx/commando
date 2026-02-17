//! Input validation

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationState {
    Valid,
    Invalid(String),
    Warning(String),
}

pub struct Validator;

impl Validator {
    pub fn validate_length(input: &str, max: Option<usize>) -> ValidationState {
        let len = input.chars().count();
        
        match max {
            Some(max_len) if len > max_len => {
                ValidationState::Invalid(format!("Maximum length is {}", max_len))
            }
            Some(max_len) if len > max_len * 80 / 100 => {
                ValidationState::Warning(format!("{}/{} characters", len, max_len))
            }
            _ => ValidationState::Valid,
        }
    }

    pub fn apply_custom<F>(input: &str, validator: Option<&F>) -> ValidationState
    where
        F: Fn(&str) -> Result<(), String>,
    {
        if let Some(validate) = validator {
            match validate(input) {
                Ok(()) => ValidationState::Valid,
                Err(msg) => ValidationState::Invalid(msg),
            }
        } else {
            ValidationState::Valid
        }
    }

    pub fn is_ascii_printable(c: char) -> bool {
        c.is_ascii_graphic() || c == ' '
    }
}
