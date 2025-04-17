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

    // Fallback return, if none of the commands match
    0
}