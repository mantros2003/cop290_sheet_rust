use crate::display::generate_column_label;
use crate::extensions::app::Mode;
use crate::extensions::app::{App, AppCommand};
use crate::{evaluator::evaluator, parser::parse};
use crate::extensions::command_handler::handle;
use crossterm::event::{self, Event, KeyCode};

pub fn handle_input(app: &mut App) -> std::io::Result<Option<AppCommand>> {
    if let Event::Key(key_event) = event::read()? {
        match app.mode {
            Mode::Normal => match key_event.code {
                KeyCode::Char('i') => {
                    app.mode = Mode::Insert;
                    app.input_buffer.clear();
                }
                KeyCode::Char('v') => {
                    app.mode = Mode::Select(app.selected.0, app.selected.1);
                    app.input_buffer.clear();
                }
                KeyCode::Left | KeyCode::Char('a') => app.move_left(),
                KeyCode::Right | KeyCode::Char('d') => app.move_right(),
                KeyCode::Up | KeyCode::Char('w') => app.move_up(),
                KeyCode::Down | KeyCode::Char('s') => app.move_down(),
                KeyCode::Char(':') => {
                    app.mode = Mode::NormalCommand;
                    app.input_buffer.clear();
                    app.input_buffer.push(':');
                }
                _ => {}
            },
            Mode::Insert => match key_event.code {
                KeyCode::Esc => {
                    app.mode = Mode::Normal;
                    app.input_buffer.clear();
                }
                KeyCode::Backspace => {
                    app.input_buffer.pop();
                }
                KeyCode::Char(c) => {
                    app.input_buffer.push(c);
                }
                KeyCode::Enter => {
                    if app.input_buffer.len() == 0 {
                    } else {
                        let mut cell = generate_column_label(app.selected.1 as u32);
                        cell += &((app.selected.0 + 1).to_string());

                        let ip = app.input_buffer.clone();
                        // Try i32
                        let ip_int = ip.parse::<i32>();
                        match ip_int {
                            Ok(_) => {
                                cell += "=";
                                cell += &ip;

                                let (mut tmp1, mut tmp2, mut tmp3): (u32, bool, bool) =
                                    (0, false, false);
                                let r = parse(&cell);
                                let ec = evaluator(r, &mut app.db, &mut tmp1, &mut tmp2, &mut tmp3);

                                if ec != 0 {
                                    app.mode = Mode::ErrMsg(crate::ERRMSG[ec as usize]);
                                    app.input_buffer.clear();
                                    return Ok(None);
                                }

                                app.input_buffer.clear();
                                app.mode = Mode::Normal;
                                return Ok(None);
                            }
                            Err(_) => {}
                        }

                        // Try f32
                        let ip_fl = ip.parse::<f32>();
                        match ip_fl {
                            Ok(f) => {
                                app.mode = Mode::ErrMsg("FLOATS UNDER PROGRESS");
                                app.input_buffer.clear();
                                return Ok(None);
                                // Implement floats in parser
                                // cell += "=";
                                // cell += &ip;

                                // let (mut tmp1, mut tmp2, mut tmp3): (u32, bool, bool) = (0, false, false);
                                // let r = parse(&cell);
                                // let _ = evaluator(r, &mut app.db, &mut tmp1, &mut tmp2, &mut tmp3);

                                // let _ = app
                                //     .db
                                //     .set_float((1000 * app.selected.1 + app.selected.0) as u32, f);

                                // app.input_buffer.clear();
                                // app.mode = Mode::Normal;
                                // return Ok(None);
                            }
                            Err(_) => {}
                        }

                        cell += &ip;

                        let (mut tmp1, mut tmp2, mut tmp3): (u32, bool, bool) = (0, false, false);
                        let r = parse(&cell);
                        let ec = evaluator(r, &mut app.db, &mut tmp1, &mut tmp2, &mut tmp3);

                        if ec != 0 {
                            app.mode = Mode::ErrMsg(crate::ERRMSG[ec as usize]);
                            app.input_buffer.clear();
                            return Ok(None);
                        }

                        app.input_buffer.clear();
                        app.mode = Mode::Normal;
                        return Ok(None);
                    }
                }
                _ => {}
            },
            Mode::NormalCommand => match key_event.code {
                KeyCode::Esc => {
                    app.input_buffer.clear();
                    app.mode = Mode::Normal;
                }
                KeyCode::Backspace => {
                    app.input_buffer.pop();
                }
                KeyCode::Char(c) => {
                    app.input_buffer.push(c);
                }
                KeyCode::Enter => {
                    match handle(app, app.input_buffer[1..].to_string().clone()) {
                        Ok(Some(val)) => { return Ok(Some(val)); }
                        Ok(None) => {},
                        Err(err) => { return Err(err); }
                    }
                }
                _ => {}
            },
            Mode::Select(r, c) => match key_event.code {
                KeyCode::Esc => {
                    app.mode = Mode::Normal;
                    app.input_buffer.clear();
                }
                KeyCode::Left | KeyCode::Char('a') => app.move_left(),
                KeyCode::Right | KeyCode::Char('d') => app.move_right(),
                KeyCode::Up | KeyCode::Char('w') => app.move_up(),
                KeyCode::Down | KeyCode::Char('s') => app.move_down(),
                KeyCode::Char('g') => {
                    app.mode = Mode::Graph((r, c), (app.selected.0, app.selected.1));
                    app.input_buffer.clear();
                }
                _ => {}
            }
            Mode::ErrMsg(_) => match key_event.kind {
                event::KeyEventKind::Press => {
                    app.mode = Mode::Normal;
                    app.input_buffer.clear();
                }
                _ => {}
            }
            Mode::Graph(_, _) => match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    app.mode = Mode::Normal;
                    app.input_buffer.clear();
                }
                _ => {},
            }
        }
    }
    Ok(None)
}