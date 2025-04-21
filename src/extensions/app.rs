use crate::database::Database;

pub enum AppCommand {
    Exit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
}

pub struct App {
    pub mode: Mode,
    pub input_buffer: String,
    pub db: Database,
    pub selected: (usize, usize),
}

impl App {
    pub fn new(db: Database) -> Self {
        Self { db, selected: (0, 0), mode: Mode::Normal, input_buffer: String::new() }
    }

    pub fn move_up(&mut self)    { if self.selected.0 > 0 { self.selected.0 -= 1; } }
    pub fn move_down(&mut self)  { self.selected.0 += 1; }
    pub fn move_left(&mut self)  { if self.selected.1 > 0 { self.selected.1 -= 1; } }
    pub fn move_right(&mut self) { self.selected.1 += 1; }

    pub fn get_value(&self, row: usize, col: usize) -> String {
        let id = (1000 * col + row) as u32;
        match self.db.get_cell(id) {
            Ok(&c) => {
                match c.get_data() {
                    Ok(data) => data.to_string(),
                    Err(()) => "ERR".to_string()
                }
            },
            Err(true) => "0".to_string(),
            Err(false) => panic!("cell out of range"),
        }
    }
}