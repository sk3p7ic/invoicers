use std::io::{self, stdout};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};

pub fn start_tui() -> io::Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_events()?;
    }
    Ok(())
}

fn handle_events() -> io::Result<bool> {
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

fn ui(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("Hello, world!").block(Block::bordered().title("Greeting")),
        frame.size(),
    );
}
