use std::io::{self, stdout};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;

use crate::{
    data::application::App,
    ui::form::ui,
};

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
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press && !app.editing => {
            Ok(handle_key_press(key_event, app))
        }
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press && app.editing => {
            handle_editing_press(key_event, app);
            Ok(false)
        }
        _ => Ok(false),
    }
}

fn handle_key_press(key_event: KeyEvent, app: &mut App) -> bool {
    match key_event.code {
        KeyCode::Char('q') => true,
        kc => {
            match kc {
                KeyCode::Tab | KeyCode::Char('j') => app.incr_selected_field(),
                KeyCode::BackTab | KeyCode::Char('k') => app.decr_selected_field(),
                KeyCode::Enter | KeyCode::Char('i') => app.editing = true,
                _ => {}
            };
            false
        }
    }
}

fn handle_editing_press(key_event: KeyEvent, app: &mut App) {
    app.edit_selected_field(key_event.code);
}
