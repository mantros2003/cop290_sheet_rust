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
    Graph((usize, usize), (usize, usize)),
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

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a default App with a mock database
    fn create_test_app(rows: u16, cols: u16) -> App<'static> {
        let db = Database::new(rows, cols);
        App::new(db)
    }

    #[test]
    fn test_app_new() {
        let app = create_test_app(10, 5);

        assert_eq!(app.mode, Mode::Normal);
        assert_eq!(app.input_buffer, "");
        assert_eq!(app.file_name, "");
        assert_eq!(app.topleft, (0, 0));
        assert_eq!(app.selected, (0, 0));
        assert_eq!(app.dissz, (0, 0));
        assert_eq!(app.db.num_rows, 10);
        assert_eq!(app.db.num_cols, 5);
    }

    #[test]
    fn test_move_up() {
        let mut app = create_test_app(10, 5);

        app.selected = (1, 0);
        app.dissz = (5, 5);
        app.move_up();
        assert_eq!(app.selected, (0, 0));
        assert_eq!(app.topleft, (0, 0));

        app.move_up();
        assert_eq!(app.selected, (0, 0));
        assert_eq!(app.topleft, (0, 0));

        // Test moving up when selected reaches top of display area
        let mut app_scroll = create_test_app(10, 5);
        app_scroll.selected = (5, 0);
        app_scroll.topleft = (5, 0);
        app_scroll.set_dissz((5, 5));

        app_scroll.move_up();
        assert_eq!(app_scroll.selected, (4, 0));
        assert_eq!(app_scroll.topleft, (4, 0));
    }

    #[test]
    fn test_move_down() {
        let mut app = create_test_app(10, 5);

        app.selected = (8, 0);
        app.set_dissz((5, 5));
        app.topleft = (4, 0);

        app.move_down();
        assert_eq!(app.selected, (9, 0));
        // TODO: Check with (5, 0)
        // Refer ui.rs for code setting display size
        assert_eq!(app.topleft, (4, 0));

        app.move_down();
        assert_eq!(app.selected, (9, 0));
        assert_eq!(app.topleft, (4, 0));

        app.selected = (4, 0);
        app.topleft = (0, 0);
        app.set_dissz((5, 5));

        app.move_down();
        // Check both of these
        assert_eq!(app.selected, (5, 0));
        assert_eq!(app.topleft, (0, 0));
    }

    #[test]
    fn test_move_left() {
        let mut app = create_test_app(10, 5);
        app.selected = (0, 3); // Start somewhere in the middle
        app.topleft = (0, 0);
        app.set_dissz((10, 5)); // Display size larger than selection area

        app.move_left();
        assert_eq!(app.selected, (0, 2));
        assert_eq!(app.topleft, (0, 0)); // topleft shouldn't change

        app.selected = (0, 1); // Move close to the left
        app.move_left();
        assert_eq!(app.selected, (0, 0));
        assert_eq!(app.topleft, (0, 0));

        app.move_left(); // Move from the left boundary
        assert_eq!(app.selected, (0, 0)); // Should stay at 0
        assert_eq!(app.topleft, (0, 0));

        // Test moving left when selected reaches left of display area
        let mut app_scroll = create_test_app(10, 5);
        app_scroll.selected = (0, 3);
        app_scroll.topleft = (0, 3); // topleft starts at selected
        app_scroll.set_dissz((5, 3)); // Display 3 columns

        app_scroll.move_left();
        assert_eq!(app_scroll.selected, (0, 2));
        assert_eq!(app_scroll.topleft, (0, 2)); // topleft should scroll left
    }

    #[test]
    fn test_move_right() {
        let mut app = create_test_app(10, 5);
        app.selected = (0, 0); // Start at the left
        app.topleft = (0, 0);
        app.set_dissz((5, 3)); // Display 3 columns

        app.move_right();
        assert_eq!(app.selected, (0, 1));
        assert_eq!(app.topleft, (0, 0)); // topleft shouldn't change

        app.selected = (0, 3); // Move close to the right
        app.set_dissz((5, 3)); // Display 3 columns
        app.topleft = (0, 2); // Adjust topleft for this scenario

        app.move_right();
        assert_eq!(app.selected, (0, 4));
        // selected (4) is dissz.1 (3) + topleft.1 (2) = 5, so topleft scrolls
        assert_eq!(app.topleft, (0, 2));

        app.move_right(); // Move from the right boundary
        assert_eq!(app.selected, (0, 4)); // Should stay at the right
        assert_eq!(app.topleft, (0, 2)); // topleft shouldn't change further

        app.selected = (0, 2);
        app.topleft = (0, 0);
        app.set_dissz((5, 3)); // Display 3 columns

        app.move_right();
        assert_eq!(app.selected, (0, 3));
        assert_eq!(app.topleft, (0, 0)); // Check this!!!!!!
    }

    #[test]
    fn test_set_dissz() {
        let mut app = create_test_app(10, 5);
        assert_eq!(app.dissz, (0, 0));

        app.set_dissz((20, 15));
        assert_eq!(app.dissz, (20, 15));

        app.set_dissz((5, 5));
        assert_eq!(app.dissz, (5, 5));
    }

    #[test]
    #[should_panic(expected = "cell out of range")]
    fn test_get_value_out_of_range() {
        let app = create_test_app(10, 5);
        // Access a cell outside the mock database dimensions (10 rows, 5 cols)
        app.get_value(10, 0); // Row out of range
    }

    #[test]
    fn test_get_value_empty_cell() {
        let app = create_test_app(10, 5);
        // MockDatabase is set up to return Err(true) for cell ID 1001 (col 1, row 1)
        let value = app.get_value(1, 1);
        assert_eq!(value, "0");
    }

    #[test]
    fn test_get_value_data_error() {
        let app = create_test_app(10, 5);
        let value = app.get_value(2, 2);
        assert_eq!(value, "0");
    }

    #[test]
    fn test_get_value_success() {
        let mut app = create_test_app(10, 5);
        let value = app.get_value(0, 0);
        assert_eq!(value, "0");

        let _ = app.db.set_int(3005, 3005);
        let value = app.get_value(5, 3);
        assert_eq!(value, "3005"); // The mock cell returns the ID as a string
    }
}