//! Ratatui UI adapter — rich TUI implementation of the Ui port.

mod highlight_syntax;
mod preview;
mod prompt;
mod select;
mod terminal_setup;

use crate::ports::ui::{Ui, UiError};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub use preview::PreviewBuilder;
pub use prompt::PromptBuilder;
pub use select::{SelectBuilder, SelectOption};
pub use terminal_setup::RatatuiUi as TerminalManager;

#[derive(Copy, Clone)]
pub struct RatatuiUI;

impl Ui for RatatuiUI {
    fn prompt(&self, label: &str) -> Result<String, UiError> {
        let mut ui = TerminalManager::new()?;
        let terminal = ui.terminal_mut();

        let mut builder = PromptBuilder::new(label)
            .placeholder("Type here...")
            .help_text("Enter to submit | Esc to cancel | Ctrl+C to quit")
            .allow_cancel(true);

        if label.contains("description") {
            builder = builder.max_length(72);
        }

        builder.run(terminal)
    }

    fn select<T: Clone>(
        &self,
        title: &str,
        options: Vec<(T, String, String)>,
    ) -> Result<T, UiError> {
        let mut ui = TerminalManager::new()?;
        let terminal = ui.terminal_mut();

        let options_len = options.len(); // Store length before moving

        let select_options: Vec<SelectOption<T>> = options
            .into_iter()
            .map(|(value, label, description)| SelectOption {
                value,
                label,
                description,
                details: None,
            })
            .collect();

        let mut builder = SelectBuilder::new(title)
            .options(select_options)
            .grid(2)
            .with_preview()
            .with_details()
            .vim_bindings(true)
            .wrap_navigation(true)
            .quick_jump_numbers(true);

        if options_len <= 4 {
            // Use stored length
            builder = builder.help_text("←/→ or h/l to move | Enter to select | Esc to cancel");
        } else {
            builder = builder.help_text("↑/↓ or j/k to move | 1-9 to jump | Enter to select");
        }

        let result = builder.run(terminal)?;
        Ok(result)
    }

    fn show_preview(&self, content: &str) {
        let mut ui = match TerminalManager::new() {
            Ok(ui) => ui,
            Err(_) => {
                println!("\n=== Preview ===\n");
                println!("{}", content);
                println!();
                return;
            }
        };
        let terminal = ui.terminal_mut();

        PreviewBuilder::new("Preview")
            .content(content)
            .vim_bindings(true)
            .wrap(true)
            .show_scrollbar(true)
            .page_size(10)
            .show(terminal)
            .ok();
    }

    fn println(&self, msg: &str) {
        let mut ui = match TerminalManager::new() {
            Ok(ui) => ui,
            Err(_) => {
                println!("{}", msg);
                return;
            }
        };
        let terminal = ui.terminal_mut();

        let _ = terminal.draw(|f| {
            // Remove type annotation
            let area = f.area();

            let lines: Vec<Line> = msg
                .lines()
                .map(|line| {
                    Line::from(vec![Span::styled(
                        line,
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )])
                })
                .collect();

            let paragraph = Paragraph::new(lines)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Message ")
                        .title_alignment(ratatui::layout::Alignment::Center)
                        .border_style(Style::default().fg(Color::Blue)),
                )
                .alignment(ratatui::layout::Alignment::Center)
                .wrap(Wrap { trim: true });

            let vertical_padding = (area.height.saturating_sub(5)) / 2;
            let centered_area = ratatui::layout::Rect {
                x: area.x + 2,
                y: area.y + vertical_padding,
                width: area.width.saturating_sub(4),
                height: 5,
            };

            f.render_widget(paragraph, centered_area);
        });

        std::thread::sleep(std::time::Duration::from_millis(800));
    }
}
