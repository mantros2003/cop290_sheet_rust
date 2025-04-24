use super::range::DependencyData;
use std::mem;
use std::ops::{Add, Div, Mul, Sub};

/// Enum for different types of data that a spreadsheet cell can store
#[derive(Debug, Clone, Copy)]
pub enum CellData {
    IntData(i32),
    FloatData(f32),
}

/// Struct to store data of a cell
#[derive(Debug, Clone, Copy)]
pub struct Cell {
    data: CellData,
    error: bool,
    dependencies: Option<DependencyData>,
}

impl CellData {
    pub fn to_int(&self) -> CellData {
        match self {
            CellData::IntData(i) => self.clone(),
            CellData::FloatData(f) => CellData::IntData(*f as i32),
        }
    }
}

impl std::fmt::Display for CellData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellData::IntData(i) => write!(f, "{}", i),
            CellData::FloatData(fl) => write!(f, "{:.2}", fl), // format to 2 decimal places
        }
    }
}

impl Add for CellData {
    type Output = CellData;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (CellData::IntData(a), CellData::IntData(b)) => CellData::IntData(a + b),
            (CellData::IntData(a), CellData::FloatData(b)) => CellData::FloatData(a as f32 + b),
            (CellData::FloatData(a), CellData::IntData(b)) => CellData::FloatData(a + b as f32),
            (CellData::FloatData(a), CellData::FloatData(b)) => CellData::FloatData(a + b),
        }
    }
}

impl Sub for CellData {
    type Output = CellData;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (CellData::IntData(a), CellData::IntData(b)) => CellData::IntData(a - b),
            (CellData::IntData(a), CellData::FloatData(b)) => CellData::FloatData(a as f32 - b),
            (CellData::FloatData(a), CellData::IntData(b)) => CellData::FloatData(a - b as f32),
            (CellData::FloatData(a), CellData::FloatData(b)) => CellData::FloatData(a - b),
        }
    }
}

impl Mul for CellData {
    type Output = CellData;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (CellData::IntData(a), CellData::IntData(b)) => CellData::IntData(a * b),
            (CellData::IntData(a), CellData::FloatData(b)) => CellData::FloatData(a as f32 * b),
            (CellData::FloatData(a), CellData::IntData(b)) => CellData::FloatData(a * b as f32),
            (CellData::FloatData(a), CellData::FloatData(b)) => CellData::FloatData(a * b),
        }
    }
}

impl Div for CellData {
    type Output = Result<CellData, ()>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (CellData::IntData(_), CellData::IntData(0))
            | (CellData::IntData(_), CellData::FloatData(0.0)) => Err(()),
            (CellData::FloatData(_), CellData::IntData(0))
            | (CellData::FloatData(_), CellData::FloatData(0.0)) => Err(()),

            (CellData::IntData(a), CellData::IntData(b)) => Ok(CellData::IntData(a / b)),
            (CellData::IntData(a), CellData::FloatData(b)) => Ok(CellData::FloatData(a as f32 / b)),
            (CellData::FloatData(a), CellData::IntData(b)) => Ok(CellData::FloatData(a / b as f32)),
            (CellData::FloatData(a), CellData::FloatData(b)) => Ok(CellData::FloatData(a / b)),
        }
    }
}

impl Cell {
    /// Returns a new integer cell
    pub fn new_i(data: i32) -> Cell {
        Cell {
            data: CellData::IntData(data),
            error: false,
            dependencies: None,
        }
    }

    /// Returns a new float cell
    pub fn new_f(data: f32) -> Cell {
        Cell {
            data: CellData::FloatData(data),
            error: false,
            dependencies: None,
        }
    }

    pub fn set_data(&mut self, data: CellData) {
        self.data = data;
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
        if self.has_error() {
            Err(())
        } else {
            Ok(&self.data)
        }
    }

    /// Sets the error value of a cell
    pub fn set_error(&mut self, err: bool) -> () {
        self.error = err;
    }

    /// Checks if the cell has error
    pub fn has_error(&self) -> bool {
        self.error
    }

    pub fn has_dep(&self) -> bool {
        if let None = self.dependencies {
            false
        } else {
            true
        }
    }

    pub fn get_dep(&self) -> Option<DependencyData> {
        self.dependencies
    }

    pub fn modify_dep(&mut self, dep: DependencyData) -> Option<DependencyData> {
        match self.dependencies.as_mut() {
            None => {
                self.dependencies = Some(dep);
                None
            }
            Some(_ret) => mem::replace(&mut self.dependencies, Some(dep)),
        }
    }

    pub fn rem_dep(&mut self) {
        self.dependencies = None;
    }
}
