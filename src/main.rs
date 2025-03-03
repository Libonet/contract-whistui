use std::io;

use color_eyre::eyre::Context;
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::{
    app::{App, AppResult},
    event::{Event, EventHandler},
    handler::handle_key_events,
    tui::Tui,
};

pub mod app;
pub mod cards;
pub mod event;
pub mod handler;
pub mod tui;
pub mod ui;
pub mod server;

#[tokio::main]
async fn main() -> AppResult<()> {
    color_eyre::install()?;

    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init().wrap_err("failed setting up the terminal")?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app).wrap_err("drawing failed!!!")?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit().wrap_err("failed restoring the terminal")?;
    Ok(())
}
