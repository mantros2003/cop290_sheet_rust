use crate::database::range::{DependencyData, DependencyNums, DependencyObject};
use crate::parser::Response;
use crate::database::Database;

pub fn evaluator(
    r: Response,
    db: &mut Database,
    topleft: &mut u32,
    running: &mut bool,
    display_state: &mut bool,
) -> i32 {
    // If parser returns an error, returns the same error code
    if r.status != 0 {
        return r.status;
    }

    // Command: q
    if r.func == 17 {
        *running = false;
        *display_state = false;
        return -1;
    }

    // Command: disable_output
    if r.func == 18 {
        *display_state = false;
        return 0;
    }

    // Command: enable_output
    if r.func == 19 {
        *display_state = true;
        return 0;
    }

    // Commands: wasd
    let mut col = *topleft / 1000;
    let mut row = *topleft % 1000;

    match r.func {
        13 => row = if row as i32 - 10 < 0 { 0 } else { row - 10 },
        14 => {
            col = if col + 20 > db.num_cols as u32 {
                if db.num_cols as i32 - 10 < 0 { 0 } else { db.num_cols as u32 - 10 }
            } else {
                col + 10
            };
        }
        15 => col = if row as i32 - 10 < 0 { 0 } else { col - 10 },
        16 => {
            row = if row + 20 > db.num_rows as u32 {
                if db.num_rows as i32 - 10 < 0 { 0 } else { db.num_rows as u32 - 10 }
            } else {
                row + 10
            };
        }
        _ => {}
    }

    *topleft = 1000 * col + row;

    if (13..=16).contains(&r.func) {
        return 0;
    }

    if !db.cell_in_range((r.target - 1001) as u32) { return 4; }

    // Command: scroll_to
    if r.func == 20 {
        *topleft = (r.target - 1001) as u32;
        return 0;
    }

    if let Ok(init) = db.is_cell_initialized((r.target - 1001) as u32) {
        if !init {
            let _ = db.set_int((r.target - 1001) as u32, 0);
        }
    }

    // let mut target;
    // if let Ok(cell) = db.get_cell_mut((r.target - 1001) as u32) { target = cell; }

    // TODO: Check and modify arg_type use
    if (r.arg_type & 2 == 1 && !db.cell_in_range((r.arg1 - 1001) as u32))
        || (r.arg_type & 1 == 1 && !db.cell_in_range((r.arg1 - 1001) as u32)) {
        return 4;
    }

    let old_error: bool;
    if let Err(val) = db.get((r.target - 1001) as u32) { old_error = val; }

    // let old_dep = db.get_cell_parent_dep((r.target - 1001) as u32);
    // if let Ok(dep) = old_dep {
    //     5;
    // }

    // let old_dep = 
    // Get old dependency
    // Then remove dependency from the cell and store a copy

    if r.func == 1 {
        let _ = db.set_int((r.target - 1001) as u32, r.arg1);
        let _ = db.set_error((r.target - 1001) as u32, false);
    }

    if r.func == 2 {
        // Check if r.args are 0 indexed or 1 indexed
        let pre = if r.arg_type & 1 == 1 { DependencyNums::new_uint(r.arg1 as u32) } else { DependencyNums::new_int(r.arg1) };
        let post = if r.arg_type & 2 == 1 { DependencyNums::new_uint(r.arg2 as u32) } else { DependencyNums::new_int(r.arg2) };
        let dep_data = DependencyData::new(r.func as u8, pre, post);

        // let dep = DependencyObject::new((r.target - 1001) as u32, r.func as u8, pre, post);

        if let Ok(cell_ref) = db.get_cell_mut((r.target - 1001) as u32) {
            cell_ref.modify_dep(dep_data);
        }
    }

    // Fallback return, if none of the commands match
    0
}