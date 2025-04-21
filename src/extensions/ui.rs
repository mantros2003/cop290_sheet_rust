use ratatui::{
    backend::Backend,
    Frame,
    layout::{Layout ,Constraint, Direction},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, Paragraph},
};
use crate::extensions::app::{App, Mode};

pub fn render<B: Backend>(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
    .direction(Direction::Vertical)
    .margin(1)
    .constraints([
        Constraint::Min(0),       // Main area (e.g., spreadsheet)
        Constraint::Length(1),    // Status bar or mode indicator
    ])
    .split(f.size());

    let header = Row::new((0..10).map(|c| format!("Col {}", c)))
        .style(Style::default().add_modifier(Modifier::BOLD));

    let rows = (0..10).map(|r| {
        let cells = (0..10).map(|c| {
            let mut cell = Cell::from(app.get_value(r, c));
            if app.selected == (r, c) {
                if app.mode == Mode::Insert {
                    cell = Cell::from(app.input_buffer.clone());
                }
                cell = cell.style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                );
            }
            cell
        });
        Row::new(cells)
    });

    let constraints = vec![Constraint::Length(10); 10];
    let table = Table::new(rows, constraints)
        .header(header)
        .block(Block::default().title("TUI Spreadsheet").borders(Borders::ALL))
        .column_spacing(1);

    f.render_widget(table, chunks[0]);

    let mode_text = match app.mode {
        Mode::Normal => "NORMAL",
        Mode::Insert => "INSERT",
    };
    
    let status_bar = Paragraph::new(mode_text)
        .style(Style::default().bg(Color::DarkGray).fg(Color::White));
    
    f.render_widget(status_bar, chunks[1]);
}
