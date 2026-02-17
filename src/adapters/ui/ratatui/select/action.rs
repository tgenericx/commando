//! User actions from key events

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Select,
    Cancel,
    Move(usize),
    ScrollPreview(i16),
}

impl Action {}
