use std::io::{self, stdout};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};

use crate::{
    data::{application::App, table::HourlyRecord},
    ui::form::ui,
};

pub fn start_tui() -> io::Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app = App::new();

    app.hours.records = vec![
        HourlyRecord {
            desc: "Hours Worked".to_string(),
            rate: 25.00,
            hours: 40f32,
        },
        HourlyRecord {
            desc: "Hours Worked".to_string(),
            rate: 25.00,
            hours: 40f32,
        },
        HourlyRecord {
            desc: "Hours Worked".to_string(),
            rate: 25.00,
            hours: 40f32,
        },
        HourlyRecord {
            desc: "Hours Worked".to_string(),
            rate: 25.00,
            hours: 40f32,
        },
    ];

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
            Ok(handle_key_press(key_event, app))
        }
        _ => Ok(false),
    }
}

fn handle_key_press(key_event: KeyEvent, app: &mut App) -> bool {
    match key_event.code {
        KeyCode::Char('q') => true,
        kc => {
            match kc {
                KeyCode::Tab => app.incr_selected_field(),
                KeyCode::BackTab => app.decr_selected_field(),
                _ => {}
            };
            false
        }
    }
}
