#![deny(clippy::all)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]
// Need it because ratatui has deps that have two
// versions of the same crate :P
#![allow(clippy::multiple_crate_versions)]

mod render;

use anyhow::Result;

fn main() -> Result<()> {
    // Set up STDOUT for displaying
    let mut terminal = ratatui::init();

    // Actually run the app
    let render_result = render::render(&mut terminal);

    // Return STDOUT to normal before exiting
    ratatui::restore();
    render_result
}
