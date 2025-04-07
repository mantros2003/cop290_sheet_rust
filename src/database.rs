use std::collections::HashMap;

pub mod cell;       // Temporarily made pub, remove it later

use cell::{Cell, CellData};

/// Struct for database
/// Data is stored in a hashmap that maps cell index to the cell struct for that cell
pub struct Database {
    pub num_rows: u16,
    pub num_cols: u16,
    store: HashMap<u32, Cell>,
}

impl Database {
    /// Makes a new instance of database with num_rows rows and num_cols columns
    pub fn new(num_rows: u16, num_cols: u16) -> Self {
        Self {
            num_rows,
            num_cols,
            store: HashMap::new(),
        }
    }

    /// Gets the data at cell represented by cell_idx
    /// returns Err(false) if cell out of range
    /// else returns Err(true) if cell has cell.error set as true
    pub fn get(&self, cell_idx: u32) -> Result<&CellData, bool> {
        if !self.cell_in_range(cell_idx) { return Err(false) }

        let cell = self.store.get(&cell_idx);
        let cell = match cell {
            Some(cell) => { cell },
            None => return Ok(&CellData::IntData(0)),
        };

        let data = cell.get_data();
        match data {
            Ok(cell_data) => { return Ok(cell_data); }
            Err(_) => { return Err(true); }
        };
    }

    pub fn set_int(&mut self, cell_idx: u32, data: i32) -> Result<(), ()> {
        if !self.cell_in_range(cell_idx) { return Err(()); }

        let cell = self.store.get_mut(&cell_idx);
        let cell = match cell {
            Some(_cell) => { _cell },
            None => {
                self.store.insert(cell_idx, Cell::new_i(data));
                return Ok(());
            },
        };
        cell.set_data_i(data);
        return Ok(());
    }

    pub fn set_float(&mut self, cell_idx: u32, data: f32) -> Result<(), ()> {
        if !self.cell_in_range(cell_idx) { return Result::Err(()); }

        let cell = self.store.get_mut(&cell_idx);
        let cell = match cell {
            Some(_cell) => { _cell },
            None => {
                self.store.insert(cell_idx, Cell::new_f(data));
                return Ok(());
            },
        };
        cell.set_data_f(data);
        return Ok(());
    }

    pub fn set_error(&mut self, cell_idx: u32, err: bool) -> Result<(), ()> {
        if !self.cell_in_range(cell_idx) { return Err(()); }

        let cell = self.store.get_mut(&cell_idx);
        let cell = match cell {
            Some(_cell) => { _cell },
            None => return Err(()),
        };
        cell.set_error(err);
        return Ok(());
    }

    fn cell_in_range(&self, cell_idx: u32) -> bool {
        if (cell_idx / 1000) >= self.num_cols.into() || (cell_idx % 1000) >= self.num_rows.into() { false }
        else { true }
    }
}

// This trait will be used for trait bounds wherever needed
// Not used till now
trait ValidCellNumber {}

impl ValidCellNumber for i32 {}
impl ValidCellNumber for f32 {}