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
            CellData::IntData(_) => self.clone(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::range::{DependencyData, DependencyNums};

    #[test]
    fn test_celldata_to_int() {
        assert!(
            matches!(CellData::IntData(10).to_int(), CellData::IntData(10)),
            "Int to Int failed"
        );
        assert!(
            matches!(CellData::FloatData(10.75).to_int(), CellData::IntData(10)),
            "Float positive to Int failed"
        );
        assert!(
            matches!(CellData::FloatData(-5.2).to_int(), CellData::IntData(-5)),
            "Float negative to Int failed"
        );
        assert!(
            matches!(CellData::FloatData(0.0).to_int(), CellData::IntData(0)),
            "Float zero to Int failed"
        );
    }

    #[test]
    fn test_celldata_display() {
        assert_eq!(
            format!("{}", CellData::IntData(-45)),
            "-45",
            "Int negative display failed"
        );
        assert_eq!(
            format!("{}", CellData::FloatData(123.454)),
            "123.45",
            "Float positive display failed (rounding down)"
        );
        assert_eq!(
            format!("{}", CellData::FloatData(0.0)),
            "0.00",
            "Float zero display failed"
        );
        assert_eq!(
            format!("{}", CellData::FloatData(-6.789)),
            "-6.79",
            "Float negative display failed (rounding up)"
        );
    }

    #[test]
    fn test_celldata_add() {
        assert!(
            matches!(CellData::IntData(5) + CellData::IntData(3), CellData::IntData(8)),
            "Int + Int failed"
        );
        assert!(
            matches!(CellData::IntData(5) + CellData::FloatData(3.5), CellData::FloatData(8.5)),
            "Int + Float failed"
        );
        assert!(
            matches!(CellData::FloatData(5.5) + CellData::IntData(3), CellData::FloatData(8.5)),
            "Float + Int failed"
        );
        assert!(
            matches!(CellData::FloatData(5.5) + CellData::FloatData(3.2), CellData::FloatData(8.7)),
            "Float + Float failed"
        );
    }

    #[test]
    fn test_celldata_sub() {
        assert!(
            matches!(CellData::IntData(5) - CellData::IntData(3), CellData::IntData(2)),
            "Int - Int failed"
        );
        assert!(
            matches!(CellData::IntData(5) - CellData::FloatData(3.5), CellData::FloatData(1.5)),
            "Int - Float failed"
        );
        assert!(
            matches!(CellData::FloatData(5.5) - CellData::IntData(3), CellData::FloatData(2.5)),
            "Float - Int failed"
        );
        assert!(
            matches!(CellData::FloatData(5.5) - CellData::FloatData(3.2), CellData::FloatData(2.3)),
            "Float - Float failed"
        );
    }

    #[test]
    fn test_celldata_mul() {
        assert!(
            matches!(CellData::IntData(5) * CellData::IntData(3), CellData::IntData(15)),
            "Int * Int failed"
        );
        assert!(
            matches!(CellData::IntData(5) * CellData::FloatData(3.5), CellData::FloatData(17.5)),
            "Int * Float failed"
        );
        assert!(
            matches!(CellData::FloatData(5.5) * CellData::IntData(3), CellData::FloatData(16.5)),
            "Float * Int failed"
        );
        assert!(
            matches!(CellData::FloatData(5.0) * CellData::FloatData(3.0), CellData::FloatData(15.0)),
            "Float * Float failed"
        );
    }

    #[test]
    fn test_celldata_div_success() {
        let epsilon = 1e-6; // Precision

        let res_int = CellData::IntData(10) / CellData::IntData(2);
        assert!(
            matches!(res_int, Ok(CellData::IntData(5))),
            "Int / Int success failed"
        );
        let res_int_trunc = CellData::IntData(10) / CellData::IntData(3);
        assert!(
            matches!(res_int_trunc, Ok(CellData::IntData(3))),
            "Int / Int truncation failed"
        );

        let res_int_float = CellData::IntData(10) / CellData::FloatData(3.0);
        if let Ok(CellData::FloatData(val)) = res_int_float {
            assert!(
                (val - 3.3333333).abs() < epsilon,
                "Int / Float failed"
            );
        } else {
            panic!("Int / Float did not return FloatData");
        }

        let res_float_int = CellData::FloatData(10.0) / CellData::IntData(3);
        if let Ok(CellData::FloatData(val)) = res_float_int {
            assert!(
                (val - 3.3333333).abs() < epsilon,
                "Float / Int failed"
            );
        } else {
            panic!("Float / Int did not return FloatData");
        }

        let res_float = CellData::FloatData(10.0) / CellData::FloatData(3.0);
        if let Ok(CellData::FloatData(val)) = res_float {
            assert!(
                (val - 3.3333333).abs() < epsilon,
                "Float / Float failed"
            );
        } else {
            panic!("Float / Float did not return FloatData");
        }
    }

    #[test]
    fn test_celldata_div_by_zero() {
        assert!(
            matches!(CellData::IntData(10) / CellData::IntData(0), Err(())),
            "Int / Int(0) failed"
        );
        assert!(
            matches!(CellData::IntData(10) / CellData::FloatData(0.0), Err(())),
            "Int / Float(0.0) failed"
        );
        assert!(
            matches!(CellData::FloatData(10.0) / CellData::IntData(0), Err(())),
            "Float / Int(0) failed"
        );
        assert!(
            matches!(CellData::FloatData(10.0) / CellData::FloatData(0.0), Err(())),
            "Float / Float(0.0) failed"
        );
        assert!(
            matches!(CellData::IntData(0) / CellData::IntData(0), Err(())),
            "Int(0) / Int(0) failed"
        );
        assert!(
            matches!(CellData::FloatData(0.0) / CellData::FloatData(0.0), Err(())),
            "Float(0.0) / Float(0.0) failed"
        );
    }

    #[test]
    fn test_cell_creation() {
        let data = CellData::IntData(100);
        let dep = Some(DependencyData::new(
            2,
            DependencyNums::U32(1001),
            DependencyNums::I32(3),
        ));

        let cell = Cell {
            data,
            error: false,
            dependencies: dep,
        };
        assert!(
            matches!(cell.data, CellData::IntData(100)),
            "Cell data creation failed"
        );
        assert_eq!(cell.error, false, "Cell error flag creation failed");
    }
}
