use ratatui::{crossterm::event::{self, Event, KeyCode, KeyEventKind}, prelude::Backend, widgets::Paragraph, Terminal};
use anyhow::Result;

pub fn render(terminal: &mut Terminal<impl Backend>) -> Result<()> {
    // Prepare the terminal for rendering
    terminal.clear()?;

    // Render loop
    loop {
        terminal.draw(|frame_handle| {
            let greeting = Paragraph::new("Hello world!");
            frame_handle.render_widget(greeting, frame_handle.area());
        })?;

        // Key press checks
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}
