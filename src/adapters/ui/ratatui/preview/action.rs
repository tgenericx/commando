//! User actions from key events

#[derive(Debug, Clone, Copy)]
pub enum Action {
    ScrollUp(u16),
    ScrollDown(u16),
    PageUp,
    PageDown,
    Home,
    End,
    Close,
}

impl Action {}
