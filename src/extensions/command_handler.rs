use crate::extensions::app::{App, AppCommand, Mode};
use crate::utils::{save_to_csv, load_from_csv};

pub fn handle(app: &mut App, ip: String) -> std::io::Result<Option<AppCommand>> {
    let args: Vec<&str> = ip.split_ascii_whitespace().collect();

    if args.len() == 1 && args[0] == "q" { return Ok(Some(AppCommand::Exit)); }

    if args[0] == "w" {
        if !args[1].ends_with(".csv") | (args.len() != 2) {
            app.mode = Mode::ErrMsg("WRONG COMMAND FORMAT, :w [file_path], file must be csv");
            return Ok(None);
        }
        
        match save_to_csv(&app.db, args[1]) {
            Ok(_) => {},
            Err(_) => {
                app.mode = Mode::ErrMsg("Failed to save file");
            }
        };
    }

    if args.len() == 2 && args[0] == "o" {
        if !args[1].ends_with(".csv") | (args.len() != 2) {
            app.mode = Mode::ErrMsg("WRONG COMMAND FORMAT, :o [file_path], file must be csv");
            return Ok(None);
        }
        
        match load_from_csv(args[1]) {
            Ok(db) => {
                app.file_name = args[1].to_string();
                app.db = db;
            },
            Err(_) => {
                app.mode = Mode::ErrMsg("Failed to load file");
                return Ok(None);
            }
        }
    }

    app.mode = Mode::Normal;
    app.input_buffer.clear();
    Ok(None)
}