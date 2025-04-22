use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::time::Duration;
use crate::extensions::app::{App, AppCommand};
use crate::extensions::app::Mode;
use crate::{parser, evaluator};

pub fn handle_input(app: &mut App) -> std::io::Result<Option<AppCommand>> {
    if let Event::Key(key_event) = event::read()? {
        match app.mode {
            Mode::Normal => match key_event.code {
                KeyCode::Char('i') => {
                    app.mode = Mode::Insert;
                    app.input_buffer.clear();
                }
                KeyCode::Esc | KeyCode::Char('q') => return Ok(Some(AppCommand::Exit)),
                KeyCode::Left => app.move_left(),
                KeyCode::Right => app.move_right(),
                KeyCode::Up => app.move_up(),
                KeyCode::Down => app.move_down(),
                KeyCode::Char(':') => {
                    app.mode = Mode::NormalCommand;
                    app.input_buffer.clear();
                }
                _ => {}
            },
            Mode::Insert => match key_event.code {
                KeyCode::Esc => {
                    // Save the input to the cell before exiting insert mode
                    // You can abstract this into a method if needed
                    let row = app.selected.0;
                    let col = app.selected.1;
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
                    if app.input_buffer.len() == 0 {}
                    else {
                        // let ip = app.input_buffer.clone();
                        // let ip_int = ip.parse::<i32>();
                        // match ip_int {
                        //     Ok(ip) => {

                        //     }
                        // }
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
                _ => {}
            }
        }
    }
    Ok(None)
}