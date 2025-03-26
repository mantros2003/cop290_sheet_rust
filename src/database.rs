use std::collections::HashMap;

// Enum for different types of data that a spreadsheet cell can store
pub enum CellData {
    IntData(i32),
    FloatData(f32),
}

// Struct to store data of a cell
struct Cell {
    data: CellData,
    error: bool,
}

// Struct for database
// Data is stored in a hashmap that maps cell index to the cell struct for that cell
pub struct Database {
    pub num_rows: u16,
    pub num_cols: u16,
    store: HashMap<u32, Cell>,
}

impl Database {
    pub fn new(num_rows: u16, num_cols: u16) -> Self {
        Self {
            num_rows,
            num_cols,
            store: HashMap::new(),
        }
    }

    fn set(&mut self, cell_idx: u32, cell: Cell) -> bool {
        self.store.insert(cell_idx, cell);
        return true;
    }

    pub fn get(&self, cell_idx: u32) -> Result<&CellData, bool> {
        if !self.cell_in_db(cell_idx) { return Err(false) }

        let cell = self.store.get(&cell_idx);
        let cell = match cell {
            Some(cell) => { cell },
            None => return Ok(&CellData::IntData(0)),
        };

        if cell.error { return Err(true) }
        Ok(&cell.data)
    }

    pub fn set_int(&mut self, cell_idx: u32, data: i32) -> Result<bool, bool> {
        if !self.cell_in_db(cell_idx) { return Result::Err(false); }

        let mut cell: Cell = Cell { data: CellData::IntData(data), error: false };
        self.set(cell_idx, cell);
        return Ok(true);
    }

    pub fn set_float(&mut self, cell_idx: u32, data: f32) -> Result<bool, bool> {
        if !self.cell_in_db(cell_idx) { return Result::Err(false); }

        let mut cell: Cell = Cell { data: CellData::FloatData(data), error: false };
        self.set(cell_idx, cell);
        return Ok(true);
    }

    fn cell_in_db(&self, cell_idx: u32) -> bool {
        if (cell_idx / 1000) >= self.num_cols.into() || (cell_idx % 1000) >= self.num_rows.into() { false }
        else { true }
    }
}

// This trait will be used for trait bounds wherever needed
// Not used till now
trait ValidCellNumber {}

impl ValidCellNumber for i32 {}
impl ValidCellNumber for f32 {}