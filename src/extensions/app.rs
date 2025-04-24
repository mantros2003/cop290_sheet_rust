use std::usize;

use crate::database::Database;

pub enum AppCommand {
    Exit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode<'a> {
    Normal,
    NormalCommand,
    Insert,
    Select(usize, usize),
    ErrMsg(&'a str),
    Graph((usize, usize), (usize, usize))
}

pub struct App<'a> {
    pub mode: Mode<'a>,
    pub input_buffer: String,
    pub db: Database,
    pub file_name: String,
    pub topleft: (usize, usize),
    pub selected: (usize, usize),
    pub dissz: (usize, usize),
}

impl<'a> App<'a> {
    pub fn new(db: Database) -> Self {
        Self {
            db,
            file_name: String::new(),
            topleft: (0, 0),
            selected: (0, 0),
            mode: Mode::Normal,
            input_buffer: String::new(),
            dissz: (0, 0),
        }
    }

    // Assume num_rows and num_cols are part of your App struct
    pub fn move_up(&mut self) {
        if self.selected.0 > 0 {
            self.selected.0 -= 1;
            if self.selected.0 < self.topleft.0 {
                self.topleft.0 -= 1;
            }
        }
    }

    pub fn move_down(&mut self) {
        if self.selected.0 + 1 < self.db.num_rows as usize {
            self.selected.0 += 1;
            if self.selected.0 > self.topleft.0 + self.dissz.0 {
                self.topleft.0 += 1;
            }
        }
    }

    pub fn move_left(&mut self) {
        if self.selected.1 > 0 {
            self.selected.1 -= 1;
            if self.selected.1 < self.topleft.1 {
                self.topleft.1 -= 1;
            }
        }
    }

    pub fn move_right(&mut self) {
        if self.selected.1 + 1 < self.db.num_cols as usize {
            self.selected.1 += 1;
            if self.selected.1 > self.topleft.1 + self.dissz.1 {
                self.topleft.1 += 1;
            }
        }
    }

    pub fn set_dissz(&mut self, sz: (usize, usize)) {
        self.dissz = sz;
    }

    pub fn get_value(&self, row: usize, col: usize) -> String {
        let id = (1000 * col + row) as u32;
        match self.db.get_cell(id) {
            Ok(&c) => match c.get_data() {
                Ok(data) => data.to_string(),
                Err(()) => "ERR".to_string(),
            },
            Err(true) => "0".to_string(),
            Err(false) => panic!("cell out of range"),
        }
    }
}
