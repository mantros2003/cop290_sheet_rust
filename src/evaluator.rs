use crate::database::range::{DependencyData, DependencyNums, DependencyObject};
use crate::parser::Response;
use crate::database::Database;
use crate::utils;

fn evaluate(cell_idx: u32) {
    todo!();
}

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

    // Capture state of the cell before modification
    // Then remove the old dependencies
    let mut old_error: bool = false;
    if let Err(val) = db.get((r.target - 1001) as u32) { old_error = val; }
    let old_dep = if let Ok(cell) = db.get_cell((r.target - 1001) as u32) { cell.get_dep() } else { None };
    db.rem_cell_parent_dep((r.target - 1001) as u32);
    db.rem_dep_dep_store((r.target - 1001) as u32);

    if r.func == 1 {
        let _ = db.set_int((r.target - 1001) as u32, r.arg1);
        let _ = db.set_error((r.target - 1001) as u32, false);
    }

    if r.func == 2 {
        // Check if r.args are 0 indexed or 1 indexed
        let pre = DependencyNums::U32((r.arg1 - 1001) as u32);
        let post = DependencyNums::I32(0);
        let dep_data = DependencyData::new(r.func as u8, pre, post);

        // let dep = DependencyObject::new((r.target - 1001) as u32, r.func as u8, pre, post);

        if let Ok(cell_ref) = db.get_cell_mut((r.target - 1001) as u32) {
            cell_ref.modify_dep(dep_data);
            db.add_dep_dep_store(DependencyObject::from_dep_data((r.target - 1001) as u32, dep_data));
        }
    }

    if r.func >= 3 && r.func <= 6 {
        // Case when both arguments are integers
        let target;
        if let Ok(cell) = db.get_cell_mut((r.target - 1001) as u32) { target = cell }
        else { panic!() };

        if (r.arg_type & 2) == 0 && (r.arg_type & 1) == 0 {
            let data = match r.func {
                3 => r.arg1 + r.arg2,
                4 => r.arg1 - r.arg2,
                5 => r.arg1 * r.arg2,
                6 => {
                    if r.arg2 == 0 {
                        target.set_error(true);
                        0
                    } else {
                        r.arg1 / r.arg2
                    }
                }
                _ => {panic!()}
            };

            target.set_data_i(data);
        } else {
            let pre;
            // Argument 1 is cell or int
            if (r.arg_type & 2) == 1 {
                pre = DependencyNums::U32((r.arg1 - 1001) as u32);
            } else {
                pre = DependencyNums::I32(r.arg1);
            }

            let post;
            // Argument 2 is cell or int
            if (r.arg_type & 1) == 1 {
                post = DependencyNums::U32((r.arg1 - 1001) as u32);
            } else {
                post = DependencyNums::I32(r.arg1);
            }

            let dep_data = DependencyData::new(r.func as u8, pre, post);

            target.modify_dep(dep_data);
            db.add_dep_dep_store(DependencyObject::from_dep_data((r.target - 1001) as u32, dep_data));
        }
    }

    // MIN, MAX, SUM, AVG, STDEV
    if r.func >= 7 && r.func <= 11 {
        let target;
        if let Ok(cell) = db.get_cell_mut((r.target - 1001) as u32) { target = cell }
        else { panic!("Target is not initialized, but should have been initialized. This only means one thing..., the world is about to end :(") };

        let pre = DependencyNums::U32((r.arg1 - 1001) as u32);
        let post = DependencyNums::U32((r.arg2 - 1001) as u32);
        let dep_data = DependencyData::new(r.func as u8, pre, post);

        target.modify_dep(dep_data);
        db.add_dep_dep_store(DependencyObject::from_dep_data((r.target - 1001) as u32, dep_data));
    }

    if r.func == 12 {
        let target;
        if let Ok(cell) = db.get_cell_mut((r.target - 1001) as u32) { target = cell }
        else { panic!("Target is not initialized, but should have been initialized. This only means one thing..., the world is about to end :(") };

        let pre;
        if r.arg_type & 2 == 1 {
            pre = DependencyNums::U32((r.arg1 - 1001) as u32);
        } else {
            pre = DependencyNums::I32(r.arg1);
        }

        let post = DependencyNums::F32(0.0);
        let dep_data = DependencyData::new(r.func as u8, pre, post);

        target.modify_dep(dep_data);
        db.add_dep_dep_store(DependencyObject::from_dep_data((r.target - 1001) as u32, dep_data));
    }

    let topo_order;
    if let Ok(vec) = utils::topological_sort(db, (r.target - 1001) as u32) { topo_order = vec; }
    else {
        let target;
        if let Ok(cell) = db.get_cell_mut((r.target - 1001) as u32) { target = cell }
        else { panic!("Target is not initialized, but should have been initialized. This only means one thing..., the world is about to end :(") };

        target.set_error(old_error);
        if let Some(dep) = old_dep {
            let _ = target.modify_dep(dep);
            db.add_dep_dep_store(DependencyObject::from_dep_data((r.target - 1001) as u32, dep));
        }

        return 3;
    }

    for cell in topo_order {
        evaluate(cell);
    }

    // Fallback return, if none of the commands match
    0
}