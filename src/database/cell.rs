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
    pub fn new_i(data: i32) -> Cell {
        Cell {
            data: CellData::IntData(data),
            error: false,
        }
    }

    pub fn new_f(data: f32) -> Cell {
        Cell {
            data: CellData::FloatData(data),
            error: false,
        }
    }

    pub fn set_data<T: ValidCellNumber>(&self, data: T) -> () {
        match self.data {
            CellData::IntData(_) => self.data = CellData::IntData(data),
            CellData::FloatData(_) => self.data = CellData::FloatData(data),
        }
    }

    pub fn set_error(&mut self, err: bool) -> () { self.error = err; }

    pub fn has_error(&self) -> bool { self.error }
}

trait ValidCellNumber {}

impl ValidCellNumber for i32 {}
impl ValidCellNumber for f32 {}