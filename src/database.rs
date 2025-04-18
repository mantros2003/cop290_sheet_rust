use std::collections::HashMap;

pub mod cell;
pub mod range;
pub mod dep_store;

use cell::{Cell, CellData};

use range::{DependencyData, DependencyObject, DependencyNums};
use dep_store::DepStore;

/// Struct for database
/// Data is stored in a hashmap that maps cell index to the cell struct for that cell
pub struct Database {
    pub num_rows: u16,
    pub num_cols: u16,
    store: HashMap<u32, Cell>,
    deps: DepStore,
}

impl Database {
    /// Makes a new instance of database with num_rows rows and num_cols columns
    pub fn new(num_rows: u16, num_cols: u16) -> Self {
        Self {
            num_rows,
            num_cols,
            store: HashMap::new(),
            deps: DepStore::new()
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

    pub fn get_cell(&self, cell_idx: u32) -> Result<&Cell, bool>{
        if !self.cell_in_range(cell_idx) { return Err(false) }

        let cell = self.store.get(&cell_idx);
        match cell {
            Some(cell) => return Ok(cell),
            None => return Err(true),
        };
    }

    pub fn get_cell_mut(&mut self, cell_idx: u32) -> Result<&mut Cell, bool>{
        if !self.cell_in_range(cell_idx) { return Err(false) }

        let cell = self.store.get_mut(&cell_idx);
        match cell {
            Some(cell) => return Ok(cell),
            None => return Err(true),
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

    pub fn is_cell_initialized(&self, cell_idx: u32) -> Result<bool, ()> {
        if !self.cell_in_range(cell_idx) { return Err(()); }

        let cell = self.store.get(&cell_idx);
        match cell {
            Some(_) => { return Ok(true); },
            None => { return Ok(false); },
        };
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

    pub fn cell_in_range(&self, cell_idx: u32) -> bool {
        if (cell_idx / 1000) >= self.num_cols.into() || (cell_idx % 1000) >= self.num_rows.into() { false }
        else { true }
    }

    pub fn get_cell_parent_dep(&self, cell_idx: u32) -> Option<DependencyData> {
        if let Ok(cell) = self.get_cell(cell_idx) { cell.get_dep() }
        else { None }
    }

    pub fn rem_cell_parent_dep(&mut self, cell_idx: u32) {
        if let Ok(cell) = self.get_cell_mut(cell_idx) { cell.rem_dep(); }
    }

    pub fn add_dep_dep_store(&mut self, dep: DependencyObject) {
        self.deps.insert(dep);
    }

    pub fn rem_dep_dep_store(&mut self, cell_idx: u32) {
        self.deps.remove(DependencyObject::new(cell_idx, 0, DependencyNums::U32(0), DependencyNums::U32(0)));
    }

    // Children are those cells which depend on the parent cell
    pub fn cell_has_child(&self, cell_idx: u32) -> bool {
        let deps = self.deps.get_from_point(cell_idx);

        deps.len() == 0
    }

    pub fn get_cell_children(&self, cell_idx: u32) -> Vec<&DependencyObject> {
        self.deps.get_from_point(cell_idx)
    }
}

// This trait will be used for trait bounds wherever needed
// Not used till now
// trait ValidCellNumber {}

// impl ValidCellNumber for i32 {}
// impl ValidCellNumber for f32 {}