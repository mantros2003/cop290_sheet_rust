use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::time::Duration;
use crate::extensions::app::{App, AppCommand};
use crate::extensions::app::Mode;

pub fn handle_input(app: &mut App) -> std::io::Result<Option<AppCommand>> {
    if let Event::Key(key_event) = event::read()? {
        match app.mode {
            Mode::Normal => match key_event.code {
                KeyCode::Char('i') => {
                    app.mode = Mode::Insert;
                    app.input_buffer.clear();  // Start fresh
                }
                KeyCode::Esc | KeyCode::Char('q') => return Ok(Some(AppCommand::Exit)),
                KeyCode::Left => app.selected.1 = app.selected.1.saturating_sub(1),
                KeyCode::Right => app.selected.1 += 1,
                KeyCode::Up => app.selected.0 = app.selected.0.saturating_sub(1),
                KeyCode::Down => app.selected.0 += 1,
                _ => {}
            },
            Mode::Insert => match key_event.code {
                KeyCode::Esc => {
                    // Save the input to the cell before exiting insert mode
                    // You can abstract this into a method if needed
                    let row = app.selected.0;
                    let col = app.selected.1;
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