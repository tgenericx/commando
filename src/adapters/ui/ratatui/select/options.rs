//! Selectable option container

/// A generic selectable option with rich details.
#[derive(Clone)]
pub struct SelectOption<T> {
    pub value: T,
    pub label: String,
    pub description: String,
    pub details: Option<String>,
}

impl<T> SelectOption<T> {}
