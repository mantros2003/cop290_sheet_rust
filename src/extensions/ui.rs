use core::num;

use ratatui::{
    backend::Backend,
    Frame,
    layout::{Layout ,Constraint, Direction},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, Paragraph},
};
use crate::extensions::app::{App, Mode};
use crate::display::generate_column_label;

const ROWLABELW: u16 = 5;
const COLWIDTH: u16 = 14;

pub fn render<B: Backend>(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
    .direction(Direction::Vertical)
    .margin(1)
    .constraints([
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .split(f.size());

    let sheet_height = chunks[0].height.saturating_sub(3);

    let num_cols_max = (chunks[0].width.saturating_sub(2 + ROWLABELW)) / COLWIDTH;

    let max_row = (app.topleft.0 + sheet_height as usize).min(app.db.num_rows as usize);
    let max_col = (app.topleft.1 + num_cols_max as usize).min(app.db.num_cols as usize);

    let sz = (max_row - app.topleft.0 - 1, max_col - app.topleft.1 - 1);
    app.set_dissz(sz);

    // Add one extra column for row numbers
    let header = Row::new(
        std::iter::once("".to_string()) // Row label for header column
            .chain((app.topleft.1..max_col).map(|c| generate_column_label(c as u32)))
    ).style(Style::default().add_modifier(Modifier::BOLD));

    let rows = (app.topleft.0..max_row).map(|r: usize| {
        let row_number_cell = Cell::from(format!("{}", r + 1)) // row number as first cell

            .style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD));

        let data_cells = (app.topleft.1..max_col).map(|c| {
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

        // Prepend the row number cell
        Row::new(std::iter::once(row_number_cell).chain(data_cells))
    });

    let mut constraints = vec![Constraint::Length(ROWLABELW)];
    constraints.extend(vec![Constraint::Length(COLWIDTH); max_col.saturating_sub(app.topleft.1)]);
    let table = Table::new(rows, constraints)
        .header(header)
        .block(Block::default().title("TUI Spreadsheet").borders(Borders::ALL))
        .column_spacing(1);


    let mode_text = match app.mode {
        Mode::Normal | Mode::NormalCommand => "NORMAL",
        Mode::Insert => "INSERT",
    };

    let command = match app.mode {
        Mode::NormalCommand => format!("command: {}", app.input_buffer.clone()),
        _ => "".to_string()
    };

    let status_bar = Paragraph::new(format!("{mode_text} {command} selected:{:?} topleft:{:?} max_cols:{:?} max_rows:{:?}", app.selected, app.topleft, max_col, max_row))
        .style(Style::default().bg(Color::DarkGray).fg(Color::White));
    
    f.render_widget(table, chunks[0]);
    f.render_widget(status_bar, chunks[1]);
}
