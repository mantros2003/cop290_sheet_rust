use super::range::Range;
use std::mem;

/// Enum for different types of data that a spreadsheet cell can store
pub enum CellData {
    IntData(i32),
    FloatData(f32),
}

pub enum DependencyType {
    Range(Range),
    Val(CellData)
}

/// Struct to store dependencies of a cell
/// range_1 is a cell (low == high), or a range
/// range_2 is None is cell only depends on range_1, else used for binary relations
/// oper represents the function applied on range_1, or binary operator between range_1 and range_2
struct Dependencies {
    range_1: DependencyType,
    range_2: DependencyType,
    oper: u8,
}

/// Struct to store data of a cell
pub struct Cell {
    data: CellData,
    error: bool,
    dependencies: Option<Dependencies>,
}

impl std::fmt::Display for CellData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellData::IntData(i) => write!(f, "{}", i),
            CellData::FloatData(fl) => write!(f, "{:.2}", fl), // format to 2 decimal places
        }
    }
}

impl Cell {
    /// Returns a new integer cell
    pub fn new_i(data: i32) -> Cell {
        Cell {
            data: CellData::IntData(data),
            error: false,
            dependencies: None
        }
    }

    /// Returns a new float cell
    pub fn new_f(data: f32) -> Cell {
        Cell {
            data: CellData::FloatData(data),
            error: false,
            dependencies: None
        }
    }

    /// Sets int data
    pub fn set_data_i(&mut self, data: i32) -> () {
        self.data = CellData::IntData(data);
    }

    /// Sets float data
    pub fn set_data_f(&mut self, data: f32) -> () {
        self.data = CellData::FloatData(data);
    }

    /// Get data from the cell
    pub fn get_data(&self) -> Result<&CellData, ()> {
        if self.has_error() { Err(()) }
        else { Ok(&self.data) }
    }

    /// Sets the error value of a cell
    pub fn set_error(&mut self, err: bool) -> () { self.error = err; }

    /// Checks if the cell has error
    pub fn has_error(&self) -> bool { self.error }

    pub fn modify_dep(&mut self, dep: Dependencies) -> Option<Dependencies> {
        match self.dependencies.as_mut() {
            None => {
                self.dependencies = Some(dep);
                None
            }
            Some(ret) => {
                mem::replace(&mut self.dependencies, Some(dep))
            }
        }
    }
}

trait ValidCellNumber {}

impl ValidCellNumber for i32 {}
impl ValidCellNumber for f32 {}