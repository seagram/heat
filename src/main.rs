mod app;
mod data;
mod storage;
mod ui;

use std::io;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;

use app::{App, InputMode};

fn main() -> io::Result<()> {
    let app_data = storage::load_data()?;
    let mut app = App::new(app_data);

    // Setup terminal
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    // Main loop
    let result = run(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui::render(frame, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => app.quit(),
                        KeyCode::Char('j') => app.select_next(),
                        KeyCode::Char('k') => app.select_previous(),
                        KeyCode::Char('g') => app.select_first(),
                        KeyCode::Char('G') => app.select_last(),
                        KeyCode::Char('a') => app.start_adding(),
                        KeyCode::Enter => {
                            app.toggle_today();
                            storage::save_data(&app.data)?;
                        }
                        _ => {}
                    },
                    InputMode::Adding => match key.code {
                        KeyCode::Enter => {
                            app.confirm_add();
                            storage::save_data(&app.data)?;
                        }
                        KeyCode::Esc => app.cancel_input(),
                        KeyCode::Backspace => {
                            app.input_buffer.pop();
                        }
                        KeyCode::Char(c) => {
                            app.input_buffer.push(c);
                        }
                        _ => {}
                    },
                    InputMode::Renaming => {}
                    InputMode::Deleting => {}
                }
            }
        }

        if app.should_quit {
            storage::save_data(&app.data)?;
            return Ok(());
        }
    }
}
