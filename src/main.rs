use std::io;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

mod tui;
mod data {
    pub mod address;
    pub mod application;
    pub mod table;
}
mod ui {
    pub mod form;
}

fn init_terminal() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;

    tui::start_tui()?;

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn main() {
    init_terminal().unwrap_or_else(|err| eprintln!("Error: {err}"))
}
