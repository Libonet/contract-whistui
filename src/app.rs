//use std::error;

use crate::{cards::Card, server::Id};
use serde::{Deserialize, Serialize};

/// Application result type.
pub type AppResult<T> = color_eyre::Result<T>;

#[derive(Debug)]
pub enum Screen {
    MainMenu,
    Lobby(LobbyInfo),
    Game(GameInfo),
}

#[derive(Debug)]
pub enum Popup {
    Exiting,
    TextBox(String),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    Playing,
    Chatting,
    Idle,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LobbyState {
    Chatting,
    Readying,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Message {
    GameStart,
    Rename(String),

    Play(Card),
    NewRound(Game),
    RequestPlay,
    TimeOut(Card),

    Ok,
    Err(String),
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub popups: Option<Popup>,
    /// Screen the user is looking at
    pub current_screen: Screen,
    pub player: Player,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            popups: None,
            current_screen: Screen::MainMenu,
            player: Player::default(),
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

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Player {
    pub name: String,
}

impl Player {
    pub fn new(id: Id) -> Self {
        // max 7 players
        let name = match id {
            0 => "Capybara",
            1 => "Anaconda",
            2 => "Axolotl",
            3 => "Iguana",
            4 => "Zebra",
            5 => "Gorilla",
            6 => "Tiger",
            _ => "Error???",
        }.to_string();
        Self { name }
    }
}

#[derive(Debug)]
pub struct LobbyInfo {
    pub chat: Chat,
    pub player_list: Vec<Player>,
    pub server: Server,
    pub host: Player,

    pub state: LobbyState,
    pub last_state: LobbyState,
}

impl LobbyInfo {
    pub fn new(chat: Chat, player_list: Vec<Player>, server: Server, host: Player) -> Self {
        Self {
            chat,
            player_list,
            server,
            host,
            state: LobbyState::Readying,
            last_state: LobbyState::Readying,
        }
    }

    pub fn change_state_to(&mut self, state: LobbyState) {
        self.last_state = self.state;
        self.state = state;
    }

    pub fn toggle_state(&mut self) {
        std::mem::swap(&mut self.state, &mut self.last_state);
    }
}

#[derive(Debug)]
pub struct GameInfo {
    pub game: Game,
    pub server: Server,
    pub chat: Chat,

    pub state: GameState,
    pub last_state: GameState,
}

impl GameInfo {
    pub fn new(game: Game, server: Server, chat: Chat) -> Self {
        Self {
            game,
            server,
            chat,
            state: GameState::Idle,
            last_state: GameState::Idle,
        }
    }

    pub fn change_state_to(&mut self, state: GameState) {
        self.last_state = self.state;
        self.state = state;
    }

    pub fn toggle_state(&mut self) {
        std::mem::swap(&mut self.state, &mut self.last_state);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Game {
    pub hand: Vec<Card>,
    pub table: Vec<Card>,
    pub triumph: Option<Card>,
}

#[derive(Debug)]
pub struct Chat {
    pub history: Vec<String>,
    pub curr_input: String,
    pub stream: tokio::net::TcpStream,
}

impl Chat {
    pub fn new(stream: tokio::net::TcpStream) -> Self {
        Self {
            history: Vec::new(),
            curr_input: String::new(),
            stream,
        }
    }
}

#[derive(Debug)]
pub struct Server {
    pub stream: tokio::net::TcpStream,
}
