#![deny(clippy::all)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]

use std::io::{stdout, Result};
use ratatui::{backend::CrosstermBackend, crossterm::{terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand}, Terminal};

fn main() -> Result<()> {
    let mut stdout_handle = stdout();
    stdout_handle.execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout_handle))?;
    terminal.clear()?;

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
