use crate::extensions::app::{App, AppCommand, Mode};
use crate::utils::{load_from_csv, save_to_csv};

pub fn handle(app: &mut App, ip: String) -> std::io::Result<Option<AppCommand>> {
    let args: Vec<&str> = ip.split_ascii_whitespace().collect();

    if args.len() == 0 {
        return Ok(None);
    }

    if args.len() == 1 {
        if args[0] == "q" {
            return Ok(Some(AppCommand::Exit));
        } else if args[0] == "w" {
            if app.file_name == "" {
                app.mode = Mode::ErrMsg(
                    "NO FILE OPENED, GIVE A PATH TO SAVE, :w [file_path], file must be csv",
                );
                return Ok(None);
            } else {
                let res = save_to_csv(&app.db, &app.file_name);
                match res {
                    Ok(()) => {
                        app.mode = Mode::Normal;
                        app.input_buffer.clear();
                    }
                    Err(_) => {
                        app.mode = Mode::ErrMsg("Failed to save");
                        return Ok(None);
                    }
                }
            }
        } else if args[0] == "wq" {
            if app.file_name == "" {
                app.mode = Mode::ErrMsg(
                    "NO FILE OPENED, GIVE A PATH TO SAVE, :w [file_path], file must be csv",
                );
                return Ok(None);
            } else {
                let res = save_to_csv(&app.db, &app.file_name);
                match res {
                    Ok(()) => {
                        app.input_buffer.clear();
                        return Ok(Some(AppCommand::Exit));
                    }
                    Err(_) => {
                        app.mode = Mode::ErrMsg("Failed to save");
                        return Ok(None);
                    }
                }
            }
        } else {
            app.mode = Mode::ErrMsg("Unrecognized command");
            return Ok(None);
        }

        return Ok(None);
    }

    if args.len() == 2 {
        if args[0] == "w" {
            if !args[1].ends_with(".csv") {
                app.mode = Mode::ErrMsg("File must be csv");
                return Ok(None);
            }

            match save_to_csv(&app.db, args[1]) {
                Ok(_) => {}
                Err(_) => {
                    app.mode = Mode::ErrMsg("Failed to save file");
                    return Ok(None);
                }
            };
        } else if args[0] == "o" {
            if !args[1].ends_with(".csv") {
                app.mode = Mode::ErrMsg("File must be csv");
                return Ok(None);
            }

            match load_from_csv(args[1]) {
                Ok(db) => {
                    app.file_name = args[1].to_string();
                    app.db = db;
                    app.selected = (0, 0);
                    app.topleft = (0, 0);
                    app.input_buffer.clear();
                    app.mode = Mode::Normal;
                }
                Err(_) => {
                    app.mode = Mode::ErrMsg("Failed to load file");
                    return Ok(None);
                }
            }
        } else {
            app.mode = Mode::ErrMsg("Unrecognized command");
            return Ok(None);
        }

        return Ok(None);
    }

    app.mode = Mode::ErrMsg("Unrecognized command");
    Ok(None)
}
