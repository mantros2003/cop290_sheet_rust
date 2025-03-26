mod database;

use database::{Database, CellData};

// Constants
const MAXROWS: u16 = 999;
const MAXCOLS: u16 = 18278;

fn main() {
    let rows: u16 = 10;
    let cols: u16 = 10;

    // Make a database
    let mut db: Database = Database::new(rows, cols);

    // Set data into the database
    for i in 0..10 {
        if i % 2 == 0 {
            let _ = db.set_int((i*1000 + i) as u32, i + 123);
        }
        else {
            let _ = db.set_float((i*1000 + i) as u32, i as f32);
        }
    }

    for i in 0..10 {
        let data = db.get(i*(1001));
        match data {
            Ok(val) => {
                let mut s = match val {
                    CellData::IntData(dat) => dat.to_string(),
                    CellData::FloatData(dat) => dat.to_string(),
                };
                println!("Cell {}:\t{s}", i * 1001);
            }
            Err(tf) => {
                if tf { println!("Cell {}:\tErr!", i * 1001) }
                else { println!("Error") }
            }
        }
    }
}
