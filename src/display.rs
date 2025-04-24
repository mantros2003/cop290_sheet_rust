use std::process;

use super::database::Database;

const CELL_WIDTH: u32 = 12;
const MAX_ROWS: u32 = 10;
const MAX_COLS: u32 = 10;

pub fn generate_column_label(mut col_index: u32) -> String {
    let mut label = String::new();
    while col_index >= 0 {
        label.push(((col_index % 26) as u8 + b'A') as char);
        if col_index < 26 {
            break;
        }
        col_index = col_index / 26 - 1;
    }
    label.chars().rev().collect()
}

pub fn print_spreadsheet(db: &Database, top_left_cell: u32) {
    let top_left_col = top_left_cell / 1000;
    let top_left_row = top_left_cell % 1000;

    let available_rows = (db.num_rows as u32).saturating_sub(top_left_row);
    let available_cols = (db.num_cols as u32).saturating_sub(top_left_col);

    let rows = available_rows.min(MAX_ROWS);
    let cols = available_cols.min(MAX_COLS);

    if rows == 0 || cols == 0 {
        eprintln!("No cells to display.");
        return;
    }

    // Print column headers (right-aligned within each cell)
    print!("{:>5}", ""); // Top-left corner space
    for j in 0..cols {
        let label = generate_column_label(top_left_col + j);
        print!("{:>width$}", label, width = CELL_WIDTH as usize);
    }
    println!();

    // Print each row
    for i in 0..rows {
        print!("{:>5}", top_left_row + i + 1); // Row label
        for j in 0..cols {
            let r = top_left_row + i;
            let c = top_left_col + j;
            match db.get(1000 * c + r) {
                Ok(&ref d) => print!(
                    "{:>width$}",
                    d.to_int().to_string(),
                    width = CELL_WIDTH as usize
                ),
                Err(true) => print!("{:>width$}", "ERR", width = CELL_WIDTH as usize),
                Err(false) => process::exit(1),
            }
        }
        println!();
    }
}
