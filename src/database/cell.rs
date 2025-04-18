use super::range::DependencyData;
use std::mem;

/// Enum for different types of data that a spreadsheet cell can store
#[derive(Debug, Clone, Copy)]
pub enum CellData {
    IntData(i32),
    FloatData(f32),
}

/// Struct to store data of a cell
#[derive(Debug)]
pub struct Cell {
    data: CellData,
    error: bool,
    dependencies: Option<DependencyData>,
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

    pub fn get_dep(&self) -> Option<DependencyData> { self.dependencies }

    pub fn modify_dep(&mut self, dep: DependencyData) -> Option<DependencyData> {
        match self.dependencies.as_mut() {
            None => {
                self.dependencies = Some(dep);
                None
            }
            Some(_ret) => {
                mem::replace(&mut self.dependencies, Some(dep))
            }
        }
    }

    pub fn rem_dep(&mut self) {self.dependencies = None; }
}