mod database;
mod parser;
mod evaluator;
mod display;
mod utils;

use database::Database;
use display::print_spreadsheet;
use parser::Response;
use std::env;
use std::process;

// Constants
const MAXROWS: u16 = 999;
const MAXCOLS: u16 = 18278;
const BUFFSZ: u16 = 256;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Exactly 2 int arguments required");
        process::exit(1);
    }

    let arg1 = &args[1];
    let arg2 = &args[2];

    if arg1.len() >= 4 || arg2.len() >= 6 {
        println!("Parameters too large, num_rows must >0 <=999 and num_cols must be >0 <=18278");
        process::exit(1);
    }

    let num_rows: u16 = match arg1.parse() {
        Ok(n) => n,
        Err(_) => {
            println!("num_rows must be ab integer >0 <=999");
            process::exit(1);
        }
    };

    let num_cols: u16 = match arg2.parse() {
        Ok(n) => n,
        Err(_) => {
            println!("num_cols must be an integer >0 <=18278");
            process::exit(1);
        }
    };

    if num_rows <= 0 || num_rows > MAXROWS || num_cols <= 0 || num_cols > MAXCOLS {
        println!("Values out of range");
        process::exit(1);
    }

    // Parameters
    let mut db: Database = Database::new(num_rows, num_cols);

    let mut topleft: u32 = 0;
    let mut running: bool = true;
    let mut display_state: bool = true;

    while running {
        if display_state { print_spreadsheet(&db, topleft); }
        
        let input = utils::get_ip(BUFFSZ as usize);

        let r: Response = parser::parse(&input);
        let ec: i32 = evaluator::evaluator(r, &mut db, &mut topleft, &mut running, &mut display_state);

        if ec == -1 { continue };
    }
}