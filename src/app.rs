//use std::error;

/// Application result type.
pub type AppResult<T> = color_eyre::Result<T>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Screen {
    MainMenu,
    SearchingForLobby,
    Lobby,
    Game,
    Exiting,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
    Playing,
    Chatting,
    Idle,
    Menu,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Screen the user is looking at
    pub current_screen: Screen,
    /// What controls are available for the player
    pub user_state: State,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            current_screen: Screen::Game,
            user_state: State::Idle,
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

