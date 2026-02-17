//! List renderer (single column)

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};
use super::super::options::SelectOption;

pub struct ListRenderer;

impl ListRenderer {
    pub fn render<T>(
        f: &mut Frame,
        area: Rect,
        options: &[SelectOption<T>],
        selected: usize,
    ) {
        let items: Vec<ListItem> = options
            .iter()
            .enumerate()
            .map(|(i, opt)| {
                let prefix = if i == selected { "▶ " } else { "  " };
                ListItem::new(format!("{}{}", prefix, opt.label))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("");

        let mut state = ListState::default();
        state.select(Some(selected));
        f.render_stateful_widget(list, area, &mut state);
    }
}
