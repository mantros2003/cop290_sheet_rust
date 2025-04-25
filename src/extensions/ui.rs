use crate::display::generate_column_label;
use crate::extensions::app::{App, Mode};
use crate::utils::{centered_rect, extract_range_data, get_formula, transform_data_for_barchart};
use ratatui::layout::Alignment;
use ratatui::text::Text;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

const ROWLABELW: u16 = 5;
const COLWIDTH: u16 = 14;

pub fn render<B: Backend>(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
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
            .chain((app.topleft.1..max_col).map(|c| generate_column_label(c as u32))),
    )
    .style(Style::default().add_modifier(Modifier::BOLD));

    let rows = (app.topleft.0..max_row).map(|r: usize| {
        let row_number_cell = Cell::from(format!("{}", r + 1)) // row number as first cell
            .style(
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            );

        let data_cells = (app.topleft.1..max_col).map(|c| {
            let mut cell = Cell::from(app.get_value(r, c));
            match app.mode {
                Mode::Select(rs, cs) => {
                    let (row_start, row_end) = if rs <= app.selected.0 {
                        (rs, app.selected.0)
                    } else {
                        (app.selected.0, rs)
                    };
                    let (col_start, col_end) = if cs <= app.selected.1 {
                        (cs, app.selected.1)
                    } else {
                        (app.selected.1, cs)
                    };

                    if (row_start..=row_end).contains(&r) && (col_start..=col_end).contains(&c) {
                        cell = cell.style(
                            Style::default()
                                .fg(Color::Black)
                                .bg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        );
                    }
                }
                _ => {}
            }
            if app.selected == (r, c) {
                if app.mode == Mode::Insert {
                    let txt = app.input_buffer.clone() + "_";
                    cell = Cell::from(txt);
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
    constraints.extend(vec![
        Constraint::Length(COLWIDTH);
        max_col.saturating_sub(app.topleft.1)
    ]);

    let table = Table::new(rows, constraints)
        .header(header)
        .block(
            Block::default()
                .title("TUI Spreadsheet")
                .borders(Borders::ALL),
        )
        .column_spacing(1);

    let mode_text = match app.mode {
        Mode::Normal | Mode::NormalCommand => "NORMAL",
        Mode::Insert => "INSERT",
        Mode::Select(_, _) => "SELECT",
        Mode::ErrMsg(_) => "ERR",
        Mode::Graph(_, _) => "GRAPH",
    };

    let bottom_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(chunks[1]);

    let ip_op_bar = bottom_row[0];
    let status_bar = bottom_row[1];

    let ip_op_text = match app.mode {
        Mode::Normal => get_formula(&app.db, (1000 * app.selected.1 + app.selected.0) as u32),
        Mode::Select(_, _) => "".to_string(),
        Mode::ErrMsg(msg) => msg.to_string(),
        _ => app.input_buffer.clone() + "_",
    };

    f.render_widget(table, chunks[0]);
    match app.mode {
        Mode::ErrMsg(_) => {
            f.render_widget(
                Paragraph::new(Text::styled(
                    format!("{}", ip_op_text),
                    Style::default().fg(Color::Red),
                ))
                .alignment(Alignment::Left),
                ip_op_bar,
            );
        }
        _ => {
            f.render_widget(
                Paragraph::new(format!("{}", ip_op_text)).alignment(Alignment::Left),
                ip_op_bar,
            );
        }
    }
    f.render_widget(
        Paragraph::new(format!(
            "{}, {} | {}",
            app.selected.0, app.selected.1, mode_text
        ))
        .alignment(Alignment::Right),
        status_bar,
    );

    if let Mode::Graph(_, _) = app.mode {
        let graphing_area = centered_rect(60, 50, chunks[0]);

        let data = extract_range_data(app);
        let data = transform_data_for_barchart(&data);

        let bars: Vec<Bar> = data
            .iter()
            .map(|(label, value)| {
                Bar::default()
                    .value(*value)
                    .label(Line::from(label.clone()))
                    .text_value(format!("{value}"))
                    .style(Style::default().fg(Color::Green))
                    .value_style(Style::default().fg(Color::Black).bg(Color::Green))
            })
            .collect();

        let bar_group = BarGroup::default().bars(&bars);

        let barchart = BarChart::default()
            .block(
                Block::default()
                    .title("Bar Chart")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black)), // Set popup background color here
            )
            .data(bar_group)
            .bar_width(5);

        f.render_widget(barchart, graphing_area);
    }
}
