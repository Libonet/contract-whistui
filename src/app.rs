//use std::error;

/// Application result type.
pub type AppResult<T> = color_eyre::Result<T>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Screen {
    SearchingForLobby,
    Lobby,
    Game,
    Exiting,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// screen the user is looking at
    pub current_screen: Screen,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            current_screen: Screen::Game,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
