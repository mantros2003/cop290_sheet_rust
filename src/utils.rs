use crate::database::{cell::CellData, range::DependencyNums, Database};
use crate::display::{self, generate_column_label};
use crate::extensions::app::{App, Mode};
use csv::Writer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};

///enum VisitState defines the visit status of
///cells during traversal
///

enum VisitState {
    Visiting,
    Visited,
}

///function get_ip is designed to
///read user input from the standard input (stdin),
///with a maximum allowed size of sz - 1 characters
///

pub fn get_ip(sz: usize) -> String {
    let mut input = String::new();

    // Flush stdout to ensure any prompt is shown before user input
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();

    // Trim newline and clip to maximum allowed length (sz - 1)
    input.trim_end().chars().take(sz - 1).collect()
}

///function implements depth-first search (DFS) on
///Database , with the goal of detecting cycles and
///producing a topological ordering of nodes (if no cycles are found)
///
fn dfs(
    db: &Database,
    node: u32,
    visited: &mut HashMap<u32, VisitState>,
    result: &mut Vec<u32>,
) -> bool {
    match visited.get(&node) {
        Some(VisitState::Visiting) => return true, // cycle
        Some(VisitState::Visited) => return false, // already done
        _ => {}
    }

    visited.insert(node, VisitState::Visiting);

    for neighbor in db.get_cell_children(node) {
        if dfs(db, neighbor, visited, result) {
            return true; // propagate cycle
        }
    }

    visited.insert(node, VisitState::Visited);
    result.push(node);
    false
}

///function topological_sort performs a topological
///sort starting from a given node in Database
///
pub fn topological_sort(db: &Database, start: u32) -> Result<Vec<u32>, ()> {
    let mut visited = HashMap::new();
    let mut result = Vec::new();
    // let mut has_cycle = false;

    if dfs(db, start, &mut visited, &mut result) {
        return Err(());
    }

    result.reverse(); // reverse to get correct topological order
    Ok(result)
}

// Check: try to use this function
// pub fn rem_dep(db: &mut Database, dep: Option<DependencyData>, cell_idx: u32) {
//     match dep {
//         Some(dep) => {
//             if (dep.get_oper() <= 6) | (dep.get_oper() == 12) {
//                 match dep.get_pre() {
//                     DependencyNums::U32(u) => db.rem_dep_point(u, cell_idx),
//                     _ => {}
//                 }
//                 match dep.get_post() {
//                     DependencyNums::U32(u) => db.rem_dep_point(u, cell_idx),
//                     _ => {}
//                 }
//             } else {
//                 db.rem_dep_range(cell_idx, dep.clone());
//             };
//         }
//         None => {}
//     };
// }

#[cfg(not(tarpaulin_include))]

///function save_to_csv is used to save
///the current state of the Database into a CSV file
///
pub fn save_to_csv(db: &Database, path: &str) -> Result<(), Box<dyn Error>> {
    // Prepare the 2D vector
    let mut table: Vec<Vec<String>> =
        vec![vec![String::new(); (db.num_cols + 1) as usize]; (db.num_rows + 1) as usize];

    for row in 0..db.num_rows {
        for col in 0..db.num_cols {
            match db.get((1000 * col + row) as u32) {
                Ok(data) => {
                    table[row as usize][col as usize] = data.to_string();
                }
                Err(_) => {}
            }
        }
    }

    // Write to CSV
    let mut wtr = Writer::from_writer(File::create(path)?);
    for row in table {
        wtr.write_record(&row)?;
    }
    wtr.flush()?;
    Ok(())
}

#[cfg(not(tarpaulin_include))]
///The load_from_csv function reads a CSV file
///and populates a Database
///
pub fn load_from_csv(path: &str) -> Result<Database, Box<dyn Error>> {
    let mut db = Database::new(0, 0);
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(File::open(path)?);

    for (row_idx, result) in rdr.records().enumerate() {
        db.num_rows = db.num_rows.max(row_idx as u16 + 1);
        let record = result?;

        for (col_idx, value) in record.iter().enumerate() {
            db.num_cols = db.num_cols.max(col_idx as u16 + 1);

            if value.trim().is_empty() {
                continue;
            }

            if let Ok(int_val) = value.parse::<i32>() {
                if int_val != 0 {
                    let _ = db.set_int((1000 * col_idx + row_idx) as u32, int_val);
                }
            } else if let Ok(float_val) = value.parse::<f32>() {
                if float_val != 0.0 {
                    let _ = db.set_float((1000 * col_idx + row_idx) as u32, float_val);
                }
            } else {
                continue; // skip unparseable
            };
        }
    }

    Ok(db)
}

#[cfg(not(tarpaulin_include))]
///function get_formula function retrieves and reconstructs the
///original formula for a given cell and database
///
pub fn get_formula(db: &Database, cell_idx: u32) -> String {
    if let Ok(cell) = db.get_cell(cell_idx) {
        if let Some(dep) = cell.get_dep() {
            match dep.get_oper() {
                1 => {
                    if let Ok(data) = cell.get_data() {
                        format!("={}", data)
                    } else {
                        "=#ERROR".to_string()
                    }
                }
                2 => {
                    if let DependencyNums::U32(val) = dep.get_pre() {
                        let ans1 = display::generate_column_label((val) / 1000);
                        let ans2 = val % 1000;
                        format!("={}{}", ans1, ans2 + 1)
                    } else {
                        String::from("=ERR")
                    }
                }
                3..=6 => {
                    let op = match dep.get_oper() {
                        3 => "+",
                        4 => "-",
                        5 => "*",
                        6 => "/",
                        _ => "?",
                    };

                    match dep.get_pre() {
                        DependencyNums::U32(val1) => {
                            match dep.get_post() {
                                DependencyNums::U32(val2) => {
                                    let ans11 = display::generate_column_label((val1) / 1000);
                                    let ans21 = val1 % 1000;

                                    let ans12 = display::generate_column_label((val2) / 1000);
                                    let ans22 = val2 % 1000;

                                    // format!("={}{}", ans1 , ans2)

                                    format!("={}{}{}{}{}", ans11, ans21 + 1, op, ans12, ans22 + 1)
                                }
                                _ => {
                                    let x = dep.get_post().to_string();
                                    let ans11 = display::generate_column_label((val1) / 1000);
                                    let ans21 = val1 % 1000;

                                    // format!("={}{}", ans1 , ans2)

                                    format!("={}{}{}{}", ans11, ans21 + 1, op, x)
                                }
                            }
                        }
                        _ => {
                            match dep.get_post() {
                                DependencyNums::U32(val2) => {
                                    let x = dep.get_pre().to_string();
                                    let ans12 = display::generate_column_label((val2) / 1000);
                                    let ans22 = val2 % 1000;

                                    // format!("={}{}", ans1 , ans2)

                                    format!("={}{}{}{}", x, op, ans12, ans22 + 1)
                                }
                                _ => {
                                    // format!("={}{}", ans1 , ans2)
                                    let x = dep.get_pre().to_string();
                                    let y = dep.get_post().to_string();

                                    format!("={}{}{}", x, op, y)
                                }
                            }
                        }
                    }
                }
                7..12 => {
                    if let (DependencyNums::U32(val1), DependencyNums::U32(val2)) =
                        (dep.get_pre(), dep.get_post())
                    {
                        let ans11 = display::generate_column_label((val1) / 1000);
                        let ans21 = val1 % 1000;

                        let ans12 = display::generate_column_label((val2) / 1000);
                        let ans22 = val2 % 1000;

                        let func = match dep.get_oper() {
                            7 => "MIN",
                            8 => "MAX",
                            9 => "AVG",
                            10 => "SUM",
                            11 => "STDEV",
                            12 => "SLEEP",
                            _ => "UNKNOWN",
                        };
                        format!("={}({}{}:{}{})", func, ans11, ans21 + 1, ans12, ans22 + 1)
                    } else {
                        String::from("=ERR")
                    }
                }
                12 => match dep.get_pre() {
                    DependencyNums::U32(u) => format!(
                        "=SLEEP({}{})",
                        generate_column_label(u / 1000),
                        (u % 1000) + 1
                    ),
                    DependencyNums::I32(i) => format!("=SLEEP({})", i),
                    _ => String::from("=ERR"),
                },
                _ => String::from("=UNKNOWN"),
            }
        } else {
            match cell.get_data() {
                Ok(data) => data.to_string(),
                Err(_) => String::from("ERR"),
            }
        }
    } else {
        String::from("")
    }
}

#[cfg(not(tarpaulin_include))]

///function extract_range_data extracts numeric
///data from a rectangular range in a spreadsheet
///
pub fn extract_range_data(app: &App) -> Vec<(String, f32)> {
    if let Mode::Graph((r1, c1), (r2, c2)) = app.mode {
        let (row_low, row_high) = if r1 <= r2 { (r1, r2) } else { (r2, r1) };
        let (col_low, col_high) = if c1 <= c2 { (c1, c2) } else { (c2, c1) };

        let mut res: Vec<(String, f32)> =
            Vec::with_capacity((row_high - row_low + 1) * (col_high - col_low + 1));

        for col in col_low..=col_high {
            for row in row_low..=row_high {
                let mut cell_label = generate_column_label(col as u32);
                cell_label += &format!("{}", row + 1);

                //let cell_label = cell_label.as_str();
                match app.db.get((1000 * col + row) as u32) {
                    Ok(data) => match data {
                        CellData::IntData(i) => {
                            res.push((cell_label, *i as f32));
                        }
                        CellData::FloatData(f) => {
                            res.push((cell_label, *f));
                        }
                    },
                    Err(_) => {
                        res.push((cell_label, 0f32));
                    }
                }
            }
        }

        res
    } else {
        vec![]
    }
}

#[cfg(not(tarpaulin_include))]
//function centered_rect computes a rectangle centered inside a given parent Rect,
///occupying a specified percentage of its width and height
///
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

#[cfg(not(tarpaulin_include))]
///function transform_data_for_barchart
///prepares data for use in a bar chart
///
pub fn transform_data_for_barchart(data: &Vec<(String, f32)>) -> Vec<(&str, u64)> {
    data.iter()
        .map(|(label, value)| {
            let label_slice: &str = label.as_str();
            let value_u64 = value.round() as u64;
            (label_slice, value_u64)
        })
        .collect()
}
