//! Grid/card renderer

use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};
use super::super::options::SelectOption;

pub struct GridRenderer;

impl GridRenderer {
    pub fn render<T>(
        f: &mut Frame,
        area: Rect,
        options: &[SelectOption<T>],
        selected: usize,
        grid_cols: usize,
    ) {
        let rows = (options.len() + grid_cols - 1) / grid_cols;
        let row_height = area.height / rows as u16;

        for row in 0..rows {
            for col in 0..grid_cols {
                let idx = row * grid_cols + col;
                if idx >= options.len() {
                    continue;
                }

                let opt = &options[idx];
                let is_selected = idx == selected;

                let cell_area = Rect {
                    x: area.x + (col as u16 * (area.width / grid_cols as u16)),
                    y: area.y + (row as u16 * row_height),
                    width: area.width / grid_cols as u16,
                    height: row_height,
                };

                let border_style = if is_selected {
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::DarkGray)
                };

                let card = Block::default()
                    .borders(Borders::ALL)
                    .border_style(border_style)
                    .title(format!(" {} ", opt.label))
                    .title_alignment(Alignment::Center);

                f.render_widget(card, cell_area);

                // Render description inside card if space permits
                if cell_area.height > 3 {
                    let inner_area = Rect {
                        x: cell_area.x + 1,
                        y: cell_area.y + 2,
                        width: cell_area.width.saturating_sub(2),
                        height: cell_area.height.saturating_sub(3),
                    };

                    let desc = Paragraph::new(opt.description.as_str())
                        .style(if is_selected {
                            Style::default().fg(Color::White)
                        } else {
                            Style::default().fg(Color::Gray)
                        })
                        .alignment(Alignment::Center)
                        .wrap(ratatui::widgets::Wrap { trim: true });

                    f.render_widget(desc, inner_area);
                }
            }
        }
    }
}
