// Enum for different types of data that a spreadsheet cell can store
pub enum CellData {
    IntData(i32),
    FloatData(f32),
}

// Struct to store data of a cell
pub struct Cell {
    data: CellData,
    error: bool,
}

impl Cell {
    // Returns a new integer cell
    pub fn new_i(data: i32) -> Cell {
        Cell {
            data: CellData::IntData(data),
            error: false,
        }
    }

    // Returns a new float cell
    pub fn new_f(data: f32) -> Cell {
        Cell {
            data: CellData::FloatData(data),
            error: false,
        }
    }

    // Sets int data
    pub fn set_data_i(&mut self, data: i32) -> () {
        self.data = CellData::IntData(data);
    }

    // Sets float data
    pub fn set_data_f(&mut self, data: f32) -> () {
        self.data = CellData::FloatData(data);
    }

    // Get data from the cell
    pub fn get_data(&self) -> Result<&CellData, ()> {
        if self.has_error() { Err(()) }
        else { Ok(&self.data) }
    }

    // Sets the error value of a cell
    pub fn set_error(&mut self, err: bool) -> () { self.error = err; }

    // Checks if the cell has error
    pub fn has_error(&self) -> bool { self.error }
}

trait ValidCellNumber {}

impl ValidCellNumber for i32 {}
impl ValidCellNumber for f32 {}