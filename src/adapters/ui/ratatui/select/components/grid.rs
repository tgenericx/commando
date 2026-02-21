//! Grid/card renderer

use super::super::options::SelectOption;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};

/// Fixed cell height: top border + 1 text line + bottom border.
const CELL_HEIGHT: u16 = 3;

pub struct GridRenderer;

impl GridRenderer {
    pub fn render<T>(
        f: &mut Frame,
        area: Rect,
        options: &[SelectOption<T>],
        selected: usize,
        grid_cols: usize,
    ) {
        if options.is_empty() || grid_cols == 0 {
            return;
        }

        let rows = options.len().div_ceil(grid_cols);
        let cols = grid_cols;

        // Distribute width evenly, spreading remainder across leading columns.
        // e.g. width=10, cols=3 → [4, 3, 3] — no pixels lost.
        let base_col_width = area.width / cols as u16;
        let col_remainder = area.width % cols as u16;

        let col_widths: Vec<u16> = (0..cols)
            .map(|c| base_col_width + if (c as u16) < col_remainder { 1 } else { 0 })
            .collect();

        let col_offsets: Vec<u16> = col_widths
            .iter()
            .scan(0u16, |acc, &w| {
                let x = *acc;
                *acc += w;
                Some(x)
            })
            .collect();

        for row in 0..rows {
            let y = area.y + row as u16 * CELL_HEIGHT;

            // Don't render rows that would overflow the available area.
            if y + CELL_HEIGHT > area.y + area.height {
                break;
            }

            for col in 0..cols {
                let idx = row * cols + col;
                if idx >= options.len() {
                    continue;
                }

                let opt = &options[idx];
                let is_selected = idx == selected;

                let cell_area = Rect {
                    x: area.x + col_offsets[col],
                    y,
                    width: col_widths[col],
                    height: CELL_HEIGHT,
                };

                let border_style = if is_selected {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::DarkGray)
                };

                // Empty block — just the border, no title.
                let card = Block::default()
                    .borders(Borders::ALL)
                    .border_style(border_style);

                f.render_widget(card, cell_area);

                // Render label centered on the single inner line.
                let label_area = Rect {
                    x: cell_area.x + 1,
                    y: cell_area.y + 1,
                    width: cell_area.width.saturating_sub(2),
                    height: 1,
                };

                let label_style = if is_selected {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::DarkGray)
                };

                let label = Paragraph::new(opt.label.as_str())
                    .style(label_style)
                    .alignment(Alignment::Center);

                f.render_widget(label, label_area);
            }
        }
    }
}
