use crate::ports::ui::UiError;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{
    Frame, Terminal,
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

/// Renders a selection UI for choosing from a list of options
pub struct SelectRenderer;

#[derive(Clone)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub description: String,
}

impl SelectRenderer {
    /// Render the selection UI with highlighting and descriptions
    pub fn render<B: Backend>(
        terminal: &mut Terminal<B>,
        title: &str,
        options: &[SelectOption],
        selected_idx: usize,
    ) -> Result<(), UiError> {
        terminal
            .draw(|f| {
                let area = f.area();
                Self::draw_select(f, area, title, options, selected_idx);
            })
            .map_err(|e| UiError(e.to_string()))?;
        Ok(())
    }

    fn draw_select(
        f: &mut Frame,
        area: Rect,
        title: &str,
        options: &[SelectOption],
        selected_idx: usize,
    ) {
        // Create layout: title, list, help text
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Min(10),   // List
                Constraint::Length(3), // Help text
            ])
            .split(area);

        // Title
        let title_text = vec![Line::from(vec![Span::styled(
            title,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )])];
        let title_widget = Paragraph::new(title_text).alignment(Alignment::Left);
        f.render_widget(title_widget, chunks[0]);

        // Create list items
        let items: Vec<ListItem> = options
            .iter()
            .enumerate()
            .map(|(idx, opt)| {
                let is_selected = idx == selected_idx;

                // Style based on selection
                let style = if is_selected {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let prefix = if is_selected { "▶ " } else { "  " };

                let content = vec![Line::from(vec![
                    Span::styled(format!("{}{:<12}", prefix, opt.label), style),
                    Span::styled(
                        format!(" — {}", opt.description),
                        if is_selected {
                            Style::default().fg(Color::Black).bg(Color::Cyan)
                        } else {
                            Style::default().fg(Color::DarkGray)
                        },
                    ),
                ])];

                ListItem::new(content).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan))
                    .title(" Options "),
            )
            .highlight_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            );

        // Create a list state for the selected index
        let mut list_state = ListState::default();
        list_state.select(Some(selected_idx));

        f.render_stateful_widget(list, chunks[1], &mut list_state);

        // Help text
        let help = vec![Line::from(vec![
            Span::styled(
                "↑/↓",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" navigate  "),
            Span::styled(
                "Enter",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" select  "),
            Span::styled(
                "Esc",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" cancel"),
        ])];
        let help_widget = Paragraph::new(help).alignment(Alignment::Center).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );
        f.render_widget(help_widget, chunks[2]);
    }

    /// Run the selection UI and return the selected option
    pub fn select<B: Backend>(
        terminal: &mut Terminal<B>,
        title: &str,
        options: Vec<SelectOption>,
    ) -> Result<String, UiError> {
        let mut selected_idx = 0;

        loop {
            Self::render(terminal, title, &options, selected_idx)?;

            if let Event::Key(key) = event::read().map_err(|e| UiError(e.to_string()))? {
                match key.code {
                    KeyCode::Enter => {
                        return Ok(options[selected_idx].value.clone());
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if selected_idx > 0 {
                            selected_idx -= 1;
                        } else {
                            selected_idx = options.len() - 1; // Wrap to bottom
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if selected_idx < options.len() - 1 {
                            selected_idx += 1;
                        } else {
                            selected_idx = 0; // Wrap to top
                        }
                    }
                    KeyCode::Home => {
                        selected_idx = 0;
                    }
                    KeyCode::End => {
                        selected_idx = options.len() - 1;
                    }
                    KeyCode::Esc => {
                        return Err(UiError("Selection cancelled".to_string()));
                    }
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Err(UiError("Selection cancelled".to_string()));
                    }
                    // Support number keys for quick selection (1-9)
                    KeyCode::Char(c) if c.is_ascii_digit() => {
                        if let Some(digit) = c.to_digit(10) {
                            let idx = (digit as usize).saturating_sub(1);
                            if idx < options.len() {
                                selected_idx = idx;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
