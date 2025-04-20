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
use std::time::Duration;
use std::time::Instant;

// Constants
const MAXROWS: u16 = 999;
const MAXCOLS: u16 = 18278;
const BUFFSZ: u16 = 256;

const ERRMSG: [&str; 5] = ["ok", "parse error", "error", "cycle detected", "cells out of range"];

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let args: Vec<String> = env::args().collect();
    let run_extension;

    if args.len() == 4 {
        if args[3] == "--extension" { run_extension = true; }
        else {
            println!("Invalid flag {}", args[3]);
            process::exit(1);
        }
    } else if args.len() == 3 { run_extension = false; }
    else {
        println!("Either 2 or 3 arguments required");
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
    let mut ec: i32 = 0;

    if !run_extension {
        let mut duration: Duration = Duration::new(0, 0);
        while running {
            if display_state { print_spreadsheet(&db, topleft); }
            print!("[{:.1}] ({}) > ", duration.as_millis() as f64 / 1000f64, {ERRMSG[ec as usize]});

            let input = utils::get_ip(BUFFSZ as usize);

            let start = Instant::now();

            let r: Response = parser::parse(&input);
            //println!("{:?}", r);
            ec = evaluator::evaluator(r, &mut db, &mut topleft, &mut running, &mut display_state);

            duration = start.elapsed();

            if ec == -1 { continue };
        }
    } else {}
}