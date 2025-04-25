use crate::database::cell::{Cell, CellData};
use crate::database::range::{DependencyData, DependencyNums, DependencyObject};
use crate::database::Database;
use crate::parser::Response;
use crate::utils;
use core::{f32, panic};
use std::thread::sleep;

fn evaluate(db: &mut Database, cell_idx: u32) {
    let target;
    if let Ok(cell) = db.get_cell_mut(cell_idx) {
        target = cell
    } else {
        panic!()
    };

    if !target.has_dep() {
        return;
    }

    let dep;
    if let Some(dep_obj) = target.get_dep() {
        dep = dep_obj;
    } else {
        panic!();
    }

    //println!("{dep:?}");

    //let _ = drop(target);

    if dep.get_oper() == 2 {
        let parent;

        // Getting the parent
        // If the cell is uninitialized, we are making a new cell and assigning it to the parent var
        match db.get_cell_clone({
            match dep.get_pre() {
                DependencyNums::U32(idx) => idx,
                _ => panic!(),
            }
        }) {
            Ok(cell) => {
                parent = cell;
            }
            Err(true) => {
                parent = Cell::new_i(0);
            }
            _ => {
                panic!();
            }
        }

        let target;
        if let Ok(cell) = db.get_cell_mut(cell_idx) {
            target = cell
        } else {
            panic!()
        };

        if let Ok(data) = parent.get_data() {
            target.set_data(data.clone());
            target.set_error(false);
        } else {
            target.set_error(true);
        }
    }

    if dep.get_oper() >= 3 && dep.get_oper() <= 6 {
        let pre;
        let post;

        match dep.get_pre() {
            DependencyNums::U32(idx) => match db.get_cell_clone(idx) {
                Ok(cell) => {
                    pre = cell;
                }
                Err(true) => {
                    pre = Cell::new_i(0);
                }
                _ => {
                    panic!();
                }
            },
            DependencyNums::I32(i) => {
                pre = Cell::new_i(i);
            }
            DependencyNums::F32(f) => {
                pre = Cell::new_f(f);
            }
        }

        match dep.get_post() {
            DependencyNums::U32(idx) => match db.get_cell_clone(idx) {
                Ok(cell) => {
                    post = cell;
                }
                Err(true) => {
                    post = Cell::new_i(0);
                }
                _ => {
                    panic!();
                }
            },
            DependencyNums::I32(i) => {
                post = Cell::new_i(i);
            }
            DependencyNums::F32(f) => {
                post = Cell::new_f(f);
            }
        }

        let target;
        if let Ok(cell) = db.get_cell_mut(cell_idx) {
            target = cell
        } else {
            panic!()
        };

        if pre.has_error() | post.has_error() {
            target.set_error(true);
            return;
        }
        target.set_error(false);

        let pre_data;
        let post_data;

        if let Ok(&data) = pre.get_data() {
            pre_data = data;
        } else {
            panic!();
        }

        if let Ok(&data) = post.get_data() {
            post_data = data;
        } else {
            panic!();
        }

        match dep.get_oper() {
            3 => {
                target.set_data(pre_data + post_data);
            }
            4 => {
                target.set_data(pre_data - post_data);
            }
            5 => {
                target.set_data(pre_data * post_data);
            }
            6 => match pre_data / post_data {
                Ok(data) => {
                    target.set_data(data);
                }
                Err(_) => {
                    target.set_error(true);
                }
            },
            _ => {}
        }
    }

    match dep.get_oper() {
        7 => min_fn(db, cell_idx),
        8 => max_fn(db, cell_idx),
        9 => avg_fn(db, cell_idx),
        10 => sum_fn(db, cell_idx),
        11 => stdev_fn(db, cell_idx),
        12 => sleep_fn(db, cell_idx),
        _ => {}
    }
}

fn min_fn(db: &mut Database, cell_idx: u32) {
    let mut min_val: f32 = f32::MAX;

    let pre;
    let post;
    if let Some(dep) = db.get_cell_parent_dep(cell_idx) {
        match (dep.get_pre(), dep.get_post()) {
            (DependencyNums::U32(pred), DependencyNums::U32(posd)) => {
                pre = pred;
                post = posd;
            }
            (_, _) => {
                panic!();
            }
        }
    } else {
        panic!();
    }

    let row_low = pre % 1000;
    let row_high = post % 1000;
    let col_low = pre / 1000;
    let col_high = post / 1000;

    for col in col_low..=col_high {
        for row in row_low..=row_high {
            let data;
            let _data = db.get(1000 * col + row);
            data = match _data {
                Ok(d) => match d {
                    CellData::IntData(i) => *i as f32,
                    CellData::FloatData(f) => *f,
                },
                Err(true) => {
                    let target;
                    if let Ok(cell) = db.get_cell_mut(cell_idx) {
                        target = cell
                    } else {
                        panic!()
                    };

                    target.set_error(true);
                    return;
                }
                Err(false) => {
                    panic!();
                }
            };

            min_val = if data < min_val { data } else { min_val };
        }
    }

    let target;
    if let Ok(cell) = db.get_cell_mut(cell_idx) {
        target = cell
    } else {
        panic!()
    };

    target.set_data_f(min_val);
    target.set_error(false);
}

fn max_fn(db: &mut Database, cell_idx: u32) {
    let mut max_val: f32 = f32::MIN;

    let pre;
    let post;
    if let Some(dep) = db.get_cell_parent_dep(cell_idx) {
        match (dep.get_pre(), dep.get_post()) {
            (DependencyNums::U32(pred), DependencyNums::U32(posd)) => {
                pre = pred;
                post = posd;
            }
            (_, _) => {
                panic!();
            }
        }
    } else {
        panic!();
    }

    let row_low = pre % 1000;
    let row_high = post % 1000;
    let col_low = pre / 1000;
    let col_high = post / 1000;

    for col in col_low..=col_high {
        for row in row_low..=row_high {
            let data;
            let _data = db.get(1000 * col + row);
            data = match _data {
                Ok(d) => match d {
                    CellData::IntData(i) => *i as f32,
                    CellData::FloatData(f) => *f,
                },
                Err(true) => {
                    let target;
                    if let Ok(cell) = db.get_cell_mut(cell_idx) {
                        target = cell
                    } else {
                        panic!()
                    };

                    target.set_error(true);
                    return;
                }
                Err(false) => {
                    panic!();
                }
            };

            max_val = if data > max_val { data } else { max_val };
        }
    }

    let target;
    if let Ok(cell) = db.get_cell_mut(cell_idx) {
        target = cell
    } else {
        panic!()
    };

    target.set_data_f(max_val);
    target.set_error(false);
}

fn avg_fn(db: &mut Database, cell_idx: u32) {
    sum_fn(db, cell_idx);

    let mut avg;
    avg = match db.get(cell_idx) {
        Ok(data) => match data {
            CellData::FloatData(f) => *f,
            CellData::IntData(i) => *i as f32,
        },
        Err(true) => {
            return;
        }
        Err(false) => {
            panic!();
        }
    };

    let pre;
    let post;
    if let Some(dep) = db.get_cell_parent_dep(cell_idx) {
        match (dep.get_pre(), dep.get_post()) {
            (DependencyNums::U32(pred), DependencyNums::U32(posd)) => {
                pre = pred;
                post = posd;
            }
            (_, _) => {
                panic!();
            }
        }
    } else {
        panic!();
    }

    let row_low = pre % 1000;
    let row_high = post % 1000;
    let col_low = pre / 1000;
    let col_high = post / 1000;

    avg /= ((row_high - row_low + 1) * (col_high - col_low + 1)) as f32;

    let target;
    if let Ok(cell) = db.get_cell_mut(cell_idx) {
        target = cell
    } else {
        panic!()
    };

    target.set_data_f(avg);
    target.set_error(false);
}

fn sum_fn(db: &mut Database, cell_idx: u32) {
    let mut sum: f32 = 0.0;

    let pre;
    let post;
    if let Some(dep) = db.get_cell_parent_dep(cell_idx) {
        match (dep.get_pre(), dep.get_post()) {
            (DependencyNums::U32(pred), DependencyNums::U32(posd)) => {
                pre = pred;
                post = posd;
            }
            (_, _) => {
                panic!();
            }
        }
    } else {
        panic!();
    }

    let row_low = pre % 1000;
    let row_high = post % 1000;
    let col_low = pre / 1000;
    let col_high = post / 1000;

    for col in col_low..=col_high {
        for row in row_low..=row_high {
            let data;
            let _data = db.get(1000 * col + row);
            data = match _data {
                Ok(d) => match d {
                    CellData::IntData(i) => *i as f32,
                    CellData::FloatData(f) => *f,
                },
                Err(true) => {
                    let target;
                    if let Ok(cell) = db.get_cell_mut(cell_idx) {
                        target = cell
                    } else {
                        panic!()
                    };

                    target.set_error(true);
                    return;
                }
                Err(false) => {
                    panic!();
                }
            };

            sum += data;
        }
    }

    let target;
    if let Ok(cell) = db.get_cell_mut(cell_idx) {
        target = cell
    } else {
        panic!()
    };

    target.set_data_f(sum);
    target.set_error(false);
}

fn stdev_fn(db: &mut Database, cell_idx: u32) {
    let avg;

    avg_fn(db, cell_idx);
    avg = match db.get(cell_idx) {
        Ok(data) => {
            match data {
                CellData::FloatData(f) => f,
                _ => panic!()
            }
        },
        Err(true) => {
            let _ = db.set_error(cell_idx, true);
            return;
        }
        _ => panic!("Should have been Ok or Err(true)")
    };

    let pre;
    let post;
    if let Some(dep) = db.get_cell_parent_dep(cell_idx) {
        match (dep.get_pre(), dep.get_post()) {
            (DependencyNums::U32(pred), DependencyNums::U32(posd)) => {
                pre = pred;
                post = posd;
            }
            (_, _) => {
                panic!();
            }
        }
    } else {
        panic!();
    }

    let row_low = pre % 1000;
    let row_high = post % 1000;
    let col_low = pre / 1000;
    let col_high = post / 1000;

    let mut var: f32 = 0.0;

    for col in col_low..=col_high {
        for row in row_low..=row_high {
            let data;
            data = match db.get(1000 * col + row) {
                Ok(d) => match d {
                    CellData::IntData(i) => *i as f32,
                    CellData::FloatData(f) => *f,
                },
                Err(true) => {
                    let target;
                    if let Ok(cell) = db.get_cell_mut(cell_idx) {
                        target = cell
                    } else {
                        panic!()
                    };

                    target.set_error(true);
                    return;
                }
                Err(false) => {
                    panic!("Should have been Ok or Err(true)");
                }
            };

            var += (data - avg).powi(2);
        }

        var /= ((row_high - row_low + 1) * (col_high - col_low + 1)) as f32;
    }

    let target;
    if let Ok(cell) = db.get_cell_mut(cell_idx) {
        target = cell
    } else {
        panic!()
    };

    target.set_data_f(var.sqrt());
    target.set_error(false);
}

fn sleep_fn(db: &mut Database, cell_idx: u32) {
    let dep = db.get_cell_parent_dep(cell_idx);

    let dep = match dep {
        Some(dep) => dep,
        None => {
            panic!("Should have had a dep");
        }
    };

    let pre = dep.get_pre();

    match pre {
        DependencyNums::U32(u) => match db.get_cell_clone(u) {
            Ok(parent) => {
                let target;
                if let Ok(cell) = db.get_cell_mut(cell_idx) {
                    target = cell
                } else {
                    panic!()
                };

                match parent.get_data() {
                    Ok(d) => {
                        match d {
                            CellData::IntData(i) => {
                                if *i >= 0 {
                                    sleep(std::time::Duration::from_secs(*i as u64));
                                }
                            }
                            CellData::FloatData(f) => {
                                if *f >= 0.0 {
                                    sleep(std::time::Duration::from_secs_f32(*f));
                                }
                            }
                        }
                        target.set_data(*d);
                        target.set_error(false);
                        return;
                    }
                    Err(()) => {
                        target.set_error(true);
                        return;
                    }
                }
            }
            Err(false) => {
                panic!();
            }
            Err(true) => {
                let target;
                if let Ok(cell) = db.get_cell_mut(cell_idx) {
                    target = cell
                } else {
                    panic!()
                };
                target.set_data_i(0);
                target.set_error(false);
                return;
            }
        },
        DependencyNums::I32(i) => {
            let time = if i < 0 { 0u64 } else { i as u64 };
            sleep(std::time::Duration::from_secs(time));

            let target;
            if let Ok(cell) = db.get_cell_mut(cell_idx) {
                target = cell
            } else {
                panic!()
            };
            target.set_data_i(i);
            target.set_error(false);

            return;
        }
        DependencyNums::F32(_) => {
            panic!();
        }
    };
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
                if db.num_cols as i32 - 10 < 0 {
                    0
                } else {
                    db.num_cols as u32 - 10
                }
            } else {
                col + 10
            };
        }
        15 => col = if col as i32 - 10 < 0 { 0 } else { col - 10 },
        16 => {
            row = if row + 20 > db.num_rows as u32 {
                if db.num_rows as i32 - 10 < 0 {
                    0
                } else {
                    db.num_rows as u32 - 10
                }
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

    if !db.cell_in_range((r.target - 1001) as u32) {
        return 4;
    }

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
    if (r.arg_type & 2 != 0 && !db.cell_in_range((r.arg1 - 1001) as u32))
        || (r.arg_type & 1 != 0 && !db.cell_in_range((r.arg2 - 1001) as u32))
    {
        return 4;
    }

    // Capture state of the cell before modification
    // Then remove the old dependencies
    let mut old_error: bool = false;
    if let Err(val) = db.get((r.target - 1001) as u32) {
        old_error = val;
    }
    let old_dep = if let Ok(cell) = db.get_cell((r.target - 1001) as u32) {
        cell.get_dep()
    } else {
        None
    };
    match old_dep {
        Some(dep) => {
            if (dep.get_oper() <= 6) | (dep.get_oper() == 12) {
                match dep.get_pre() {
                    DependencyNums::U32(u) => db.rem_dep_point(u, (r.target - 1001) as u32),
                    _ => {}
                }
                match dep.get_post() {
                    DependencyNums::U32(u) => db.rem_dep_point(u, (r.target - 1001) as u32),
                    _ => {}
                }
            } else {
                db.rem_dep_range((r.target - 1001) as u32, dep.clone());
            };
        }
        None => {}
    };
    db.rem_cell_parent_dep((r.target - 1001) as u32);

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
            db.add_dep_point((r.arg1 - 1001) as u32, (r.target - 1001) as u32);
            // db.add_dep_range(DependencyObject::from_dep_data((r.target - 1001) as u32, dep_data));
        }
    }

    // Binary relations: + - * /
    if r.func >= 3 && r.func <= 6 {
        let target;
        if let Ok(cell) = db.get_cell_mut((r.target - 1001) as u32) {
            target = cell
        } else {
            panic!()
        };

        // Case when both arguments are integers
        // This will not have any operator/ dependencies
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
                _ => {
                    panic!()
                }
            };

            target.set_data_i(data);
        } else {
            let pre;
            // Argument 1 is cell or int
            if (r.arg_type & 2) != 0 {
                pre = DependencyNums::U32((r.arg1 - 1001) as u32);
            } else {
                pre = DependencyNums::I32(r.arg1);
            }

            let post;
            // Argument 2 is cell or int
            if (r.arg_type & 1) != 0 {
                post = DependencyNums::U32((r.arg2 - 1001) as u32);
            } else {
                post = DependencyNums::I32(r.arg2);
            }

            let dep_data = DependencyData::new(r.func as u8, pre, post);

            target.modify_dep(dep_data);
            match (pre, post) {
                (DependencyNums::U32(u1), DependencyNums::U32(u2)) => {
                    db.add_dep_point(u1, (r.target - 1001) as u32);
                    db.add_dep_point(u2, (r.target - 1001) as u32);
                }
                (DependencyNums::U32(u), _) => {
                    db.add_dep_point(u, (r.target - 1001) as u32);
                }
                (_, DependencyNums::U32(u)) => {
                    db.add_dep_point(u, (r.target - 1001) as u32);
                }
                (_, _) => {}
            }
        }
    }

    // MIN, MAX, SUM, AVG, STDEV
    if r.func >= 7 && r.func <= 11 {
        let target;
        if let Ok(cell) = db.get_cell_mut((r.target - 1001) as u32) {
            target = cell
        } else {
            panic!("Target is not initialized, but should have been initialized. This only means one thing..., the world is about to end :(")
        };

        let pre = DependencyNums::U32((r.arg1 - 1001) as u32);
        let post = DependencyNums::U32((r.arg2 - 1001) as u32);
        let dep_data = DependencyData::new(r.func as u8, pre, post);

        target.modify_dep(dep_data);
        db.add_dep_range(DependencyObject::from_dep_data(
            (r.target - 1001) as u32,
            dep_data,
        ));
    }

    // Command: sleep
    if r.func == 12 {
        let target;
        if let Ok(cell) = db.get_cell_mut((r.target - 1001) as u32) {
            target = cell
        } else {
            panic!("Target is not initialized, but should have been initialized. This only means one thing..., the world is about to end :(")
        };

        let post = DependencyNums::I32(0);
        let pre;

        if r.arg_type & 2 != 0 {
            pre = DependencyNums::U32((r.arg1 - 1001) as u32);
            let dep_data = DependencyData::new(r.func as u8, pre, post);
            target.modify_dep(dep_data);
            db.add_dep_point((r.arg1 - 1001) as u32, (r.target - 1001) as u32);
        } else {
            pre = DependencyNums::I32(r.arg1);
            let dep_data = DependencyData::new(r.func as u8, pre, post);
            target.modify_dep(dep_data);
        }
    }

    let topo_order;
    if let Ok(vec) = utils::topological_sort(db, (r.target - 1001) as u32) {
        topo_order = vec;
    } else {
        let target;
        if let Ok(cell) = db.get_cell_mut((r.target - 1001) as u32) {
            target = cell;
        } else {
            panic!("Panicking from dependency removal logic")
        };

        target.set_error(old_error);
        match target.get_dep() {
            Some(dep) => {
                if (dep.get_oper() <= 6) | (dep.get_oper() == 12) {
                    match dep.get_pre() {
                        DependencyNums::U32(u) => db.rem_dep_point(u, (r.target - 1001) as u32),
                        _ => {}
                    }
                    match dep.get_post() {
                        DependencyNums::U32(u) => db.rem_dep_point(u, (r.target - 1001) as u32),
                        _ => {}
                    }
                } else {
                    db.rem_dep_range((r.target - 1001) as u32, dep.clone());
                };
            }
            None => {}
        };

        let target;
        if let Ok(cell) = db.get_cell_mut((r.target - 1001) as u32) {
            target = cell;
        } else {
            panic!("Panicking from dependency removal logic")
        };

        match old_dep {
            Some(dep) => {
                target.modify_dep(dep);
            }
            None => {
                target.rem_dep();
            }
        }

        return 3;
    }

    for cell in topo_order {
        evaluate(db, cell);
    }

    // Fallback return, if none of the commands match
    0
}

#[cfg(test)]
mod tests {
    use crate::parser;
    use super::*;

    #[test]
    fn test() {
        let mut db = Database::new(100, 100);
        let mut state: (u32, bool, bool) = (0, true, true);

        let mut _r = parser::parse("A1=100");
        assert!(_r == Response{status: 0, func: 1, target: 1001, arg1: 100, arg2: 0, arg_type:  0}, "r = {:?}", _r);

        let mut _ec = evaluator(_r, &mut db, &mut state.0, &mut state.1, &mut state.2);
        assert!(_ec == 0);
        assert!(state == (0, true, true));
        assert!(db.get(0) == Ok(&CellData::IntData(100)));

        _r = parser::parse("A1=50+50");
        assert!(_r == Response{status: 0, func: 3, target: 1001, arg1: 50, arg2: 50, arg_type:  0}, "r = {:?}", _r);

        _ec = evaluator(_r, &mut db, &mut state.0, &mut state.1, &mut state.2);
        assert!(_ec == 0);
        assert!(state == (0, true, true));
        assert!(db.get(0) == Ok(&CellData::IntData(100)));

        _r = parser::parse("A1=B1+100");
        assert!(_r == Response{status: 0, func: 3, target: 1001, arg1: 2001, arg2: 100, arg_type: 2}, "r = {:?}", _r);

        _ec = evaluator(_r, &mut db, &mut state.0, &mut state.1, &mut state.2);
        assert!(_ec == 0);
        assert!(state == (0, true, true));
        assert!(db.get(0) == Ok(&CellData::IntData(100)));
        assert!(db.get_cell_parent_dep(0) == Some(DependencyData::new(3, DependencyNums::U32(1000), DependencyNums::I32(100))));

        _r = parser::parse("A1=100");
        assert!(_r == Response{status: 0, func: 1, target: 1001, arg1: 100, arg2: 0, arg_type: 0}, "r = {:?}", _r);

        _ec = evaluator(_r, &mut db, &mut state.0, &mut state.1, &mut state.2);
        assert!(_ec == 0);
        assert!(state == (0, true, true));
        assert!(db.get(0) == Ok(&CellData::IntData(100)));
        assert!(db.get_cell_parent_dep(0) == None);

        _r = parser::parse("C1=A1/B1");
        assert!(_r == Response{status: 0, func: 6, target: 3001, arg1: 1001, arg2: 2001, arg_type: 3}, "r = {:?}", _r);

        _ec = evaluator(_r, &mut db, &mut state.0, &mut state.1, &mut state.2);
        assert!(_ec == 0);
        assert!(state == (0, true, true));
        assert!(db.get(2000) == Err(true));
        assert!(db.get_cell_parent_dep(2000) == Some(DependencyData::new(6, DependencyNums::U32(0), DependencyNums::U32(1000))));

        _r = parser::parse("B1=1");
        assert!(_r == Response{status: 0, func: 1, target: 2001, arg1: 1, arg2: 0, arg_type: 0}, "r = {:?}", _r);

        _ec = evaluator(_r, &mut db, &mut state.0, &mut state.1, &mut state.2);
        assert!(_ec == 0);
        assert!(state == (0, true, true));
        assert!(db.get(2000) == Ok(&CellData::IntData(100)), "val = {:?}", db.get(2000));
        assert!(db.get_cell_parent_dep(2000) == Some(DependencyData::new(6, DependencyNums::U32(0), DependencyNums::U32(1000))));

        _r = parser::parse("B1=2");
        _ec = evaluator(_r, &mut db, &mut state.0, &mut state.1, &mut state.2);
        assert!(db.get(2000) == Ok(&CellData::IntData(50)));

        _r = parser::parse("A2=A1+30");
        _ec = evaluator(_r, &mut db, &mut state.0, &mut state.1, &mut state.2);

        _r = parser::parse("B2=A2*10");
        _ec = evaluator(_r, &mut db, &mut state.0, &mut state.1, &mut state.2);

        _r = parser::parse("C2=B2-A1");
        _ec = evaluator(_r, &mut db, &mut state.0, &mut state.1, &mut state.2);

        assert!(db.get(1) == Ok(&CellData::IntData(130)), "val = {:?}", db.get(1));
        assert!(db.get(1001) == Ok(&CellData::IntData(1300)), "val = {:?}", db.get(2000));
        assert!(db.get(2001) == Ok(&CellData::IntData(1200)), "val = {:?}", db.get(2001));

        _r = parser::parse("A1=MAX(A2:C2)");
        assert!(_r == Response{target: 1001, status: 0, func: 8, arg1: 1002, arg2: 3002, arg_type: 3});

        _ec = evaluator(_r, &mut db, &mut state.0, &mut state.1, &mut state.2);
        assert!(_ec == 3);
        assert!(state == (0, true, true));
        assert!(db.get(0) == Ok(&CellData::IntData(100)), "val = {:?}", db.get(0));
        assert!(db.get_cell_parent_dep(0) == None);

        _r = parser::parse("A3=MAX(A2:C2)");
        _ec = evaluator(_r, &mut db, &mut state.0, &mut state.1, &mut state.2);

        _r = parser::parse("B3=MIN(A2:C2)");
        _ec = evaluator(_r, &mut db, &mut state.0, &mut state.1, &mut state.2);

        _r = parser::parse("C3=SUM(A2:C2)");
        _ec = evaluator(_r, &mut db, &mut state.0, &mut state.1, &mut state.2);

        _r = parser::parse("D3=AVG(A2:C2)");
        _ec = evaluator(_r, &mut db, &mut state.0, &mut state.1, &mut state.2);

        _r = parser::parse("E3=STDEV(A2:C2)");
        _ec = evaluator(_r, &mut db, &mut state.0, &mut state.1, &mut state.2);

        assert!(db.get(2) == Ok(&CellData::FloatData(1300.0)), "val = {:?}", db.get(2));
        assert!(db.get(1002) == Ok(&CellData::FloatData(130.0)), "val = {:?}", db.get(1002));
        assert!(db.get(2002) == Ok(&CellData::FloatData(2630.0)), "val = {:?}", db.get(2002));
        assert!(db.get(3002) == Ok(&CellData::FloatData(2630.0 / 3.0)), "val = {:?}", db.get(3002));
        assert!(db.get(4002) == Ok(&CellData::FloatData(274.6071)), "val = {:?}", db.get(4002));

        _r = parser::parse("F3=SLEEP(1)");
        _ec = evaluator(_r, &mut db, &mut state.0, &mut state.1, &mut state.2);
        assert!(db.get_cell_parent_dep(5002) == Some(DependencyData::new(12, DependencyNums::I32(1), DependencyNums::I32(0))));

        _r = parser::parse("G3=SLEEP(F3)");
        _ec = evaluator(_r, &mut db, &mut state.0, &mut state.1, &mut state.2);
        assert!(db.get_cell_parent_dep(6002) == Some(DependencyData::new(12, DependencyNums::U32(5002), DependencyNums::I32(0))));

        assert!(db.get(5002) == Ok(&CellData::IntData(1)), "val = {:?}", db.get(5002));
        assert!(db.get(6002) == Ok(&CellData::IntData(1)), "val = {:?}", db.get(6002));

        _r = parser::parse("disable_output");
        _ec = evaluator(_r, &mut db, &mut state.0, &mut state.1, &mut state.2);
        assert!(state.2 == false);

        _r = parser::parse("enable_output");
        _ec = evaluator(_r, &mut db, &mut state.0, &mut state.1, &mut state.2);
        assert!(state.2 == true);

        _r = parser::parse("q");
        _ec = evaluator(_r, &mut db, &mut state.0, &mut state.1, &mut state.2);
        assert!(state.1 == false);
        assert!(_ec == -1);

        state.1 = true;

        _r = parser::parse("A4=A1+B1");
        _ec = evaluator(_r, &mut db, &mut state.0, &mut state.1, &mut state.2);

        _r = parser::parse("B4=A4-100");
        _ec = evaluator(_r, &mut db, &mut state.0, &mut state.1, &mut state.2);

        _r = parser::parse("C4=B4*A4");
        _ec = evaluator(_r, &mut db, &mut state.0, &mut state.1, &mut state.2);
        
        assert!(db.get(3) == Ok(&CellData::IntData(102)), "val = {:?}", db.get(3));
        assert!(db.get(1003) == Ok(&CellData::IntData(2)), "val = {:?}", db.get(1003));
        assert!(db.get(2003) == Ok(&CellData::IntData(204)), "val = {:?}", db.get(2003));

        _r = parser::parse("A5=B5");
        _ec = evaluator(_r, &mut db, &mut state.0, &mut state.1, &mut state.2);

        _r = parser::parse("B5=A1");
        _ec = evaluator(_r, &mut db, &mut state.0, &mut state.1, &mut state.2);

        assert!(db.get(4) == db.get(1004));
        assert!(db.get(0) == db.get(1004));
    }

    #[test]
    fn test_scroll() {
        let mut db = Database::new(100, 100);
        let mut state: (u32, bool, bool) = (0, true, true);

        let r = parser::parse("scroll_to D10");
        assert!(r == Response{status: 0, func: 20, target: 4010, arg1: 0, arg2: 0, arg_type:  0}, "r = {:?}", r);

        let ec = evaluator(r, &mut db, &mut state.0, &mut state.1, &mut state.2);
        assert!(state == (3009,true , true));
        assert!(ec == 0);
    
    }

    #[test]
    fn test_wasd() {
        let mut db = Database::new(100, 100);
        let mut state: (u32, bool, bool) = (0, true, true);

        let mut r = parser::parse("s");
        assert!(r == Response{status: 0, func: 16, target: 0, arg1: 0, arg2: 0, arg_type:  0}, "r = {:?}", r);
        let mut ec = evaluator(r, &mut db, &mut state.0, &mut state.1, &mut state.2);
        assert!(state == (10,true , true));
        assert!(ec == 0);

        r = parser::parse("d");
        assert!(r == Response{status: 0, func: 14, target: 0, arg1: 0, arg2: 0, arg_type:  0}, "r = {:?}", r);
        ec = evaluator(r, &mut db, &mut state.0, &mut state.1, &mut state.2);
        assert!(state == (10010,true , true));
        assert!(ec == 0);
        
        r = parser::parse("w");
        assert!(r == Response{status: 0, func: 13, target: 0, arg1: 0, arg2: 0, arg_type:  0}, "r = {:?}", r);
        ec = evaluator(r, &mut db, &mut state.0, &mut state.1, &mut state.2);
        assert!(state == (10000,true , true));
        assert!(ec == 0);

        r = parser::parse("a");
        assert!(r == Response{status: 0, func: 15, target: 0, arg1: 0, arg2: 0, arg_type:  0}, "r = {:?}", r);
        ec = evaluator(r, &mut db, &mut state.0, &mut state.1, &mut state.2);
        assert!(state == (0,true , true));
        assert!(ec == 0);

        state.0 = 0;
        r = parser::parse("w");
        ec = evaluator(r, &mut db, &mut state.0, &mut state.1, &mut state.2);
        assert!(state.0 == 0);
        assert!(ec == 0);

        r = parser::parse("a");
        ec = evaluator(r, &mut db, &mut state.0, &mut state.1, &mut state.2);
        assert!(state.0 == 0);
        assert!(ec == 0);
    
    }

}