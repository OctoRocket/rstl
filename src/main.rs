#![deny(clippy::all)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]
// Need it because ratatui has deps that have two
// versions of the same crate :P
#![allow(clippy::multiple_crate_versions)]

use anyhow::Result;
use ratatui::{
    crossterm::event::{
        self,
        Event,
        KeyCode,
        KeyEventKind
    },
    widgets::{
        List,
        ListDirection,
        Block,
    },
    style::{
        Style,
        Stylize,
    },
    Frame,
};

fn main() -> Result<()> {
    // Set up STDOUT for displaying
    let mut terminal = ratatui::init();
    terminal.clear()?;

    let state = ProgramState::default();

    // Actually run the app
    // Render loop
    loop {
        terminal.draw(render(&state))?;

        // Checks if "Q" has been pressed.
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    // Return STDOUT to normal before exiting
    ratatui::restore();
    Ok(())
}

/// Handles rendering a single frame.
fn render(state: &ProgramState) -> impl Fn(&mut Frame) {
    |frame_handle| {
        let items = ["Item 1", "Item 2", "Item 3"];
        let list = List::new(items)
            .block(Block::bordered().title("List"))
            .style(Style::new().white())
            .highlight_style(Style::new().italic())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::BottomToTop);
        frame_handle.render_widget(list, frame_handle.area());
    }
}

/// Singleton that represents the entire state of the program.
#[derive(Default)]
struct ProgramState {
    items: Vec<ListItem>, // List of all items on the todo list.
}

/// Represents one item on the todo list.
struct ListItem {
    completed: bool,
    text: String,
}

impl ListItem {
    fn new(text: String) -> Self {
        Self {
            completed: false,
            text,
        }
    }

    fn toggle_completed(&mut self) {
        self.completed = !self.completed;
    }
}

// Takes a users input about data related to creating a task.
// fn task_creation() {
    // Task Name, Task Info, Task Date, Task Priority, Task Color(?) - Default Color Based Of Priority (Not for this function), Task Categoriy. - This should all be moved to the doc.
// }