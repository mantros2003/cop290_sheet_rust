pub mod app;
mod input;
mod ui;
mod command_handler;

use self::app::{App, AppCommand};
use self::input::handle_input;
use self::ui::render;
use crate::database::Database;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{stdout, Stdout};

pub fn run_tui(db: Database) -> std::io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<std::io::Stdout>> = Terminal::new(backend)?;

    let mut app = App::new(db);

    loop {
        terminal.draw(|f| render::<CrosstermBackend<Stdout>>(f, &mut app))?;
        if let Some(cmd) = handle_input(&mut app)? {
            match cmd {
                AppCommand::Exit => break,
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
