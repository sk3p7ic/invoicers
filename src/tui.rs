use std::io::{self, stdout};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};

use crate::{data::application::App, ui::form::ui};

pub fn start_tui() -> io::Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app = App::new();
    run_app(&mut terminal, &mut app)
}

fn run_app<B: Backend>(term: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        term.draw(|f| ui(f, app))?;
        match handle_events(app) {
            Ok(do_quit) => match do_quit {
                true => break,
                false => {}
            },
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn handle_events(app: &mut App) -> io::Result<bool> {
    match event::read()? {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            Ok(handle_key_press(key_event))
        }
        _ => Ok(false),
    }
}

fn handle_key_press(key_event: KeyEvent) -> bool {
    match key_event.code {
        KeyCode::Char('q') => true,
        _ => false,
    }
}
