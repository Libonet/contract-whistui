//use std::error;

use crate::cards::Card;
use tokio::sync::{broadcast, oneshot};

/// Application result type.
pub type AppResult<T> = color_eyre::Result<T>;

#[derive(Debug)]
pub enum Screen {
    MainMenu,
    Lobby(LobbyInfo),
    Game(GameInfo),
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Message {
    GameStart,
    Play(Card),
    NewRound(Game),
    RequestPlay,
    TimeOut(Card),
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub is_exiting: bool,
    /// Screen the user is looking at
    pub current_screen: Screen,
    pub last_screen: Screen,
    pub player: Player,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            is_exiting: false,
            current_screen: Screen::MainMenu,
            last_screen: Screen::MainMenu,
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

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Player {
    pub name: String,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug)]
pub struct LobbyInfo {
    pub chat: Chat,
    pub player_list: Vec<String>,
    pub server: Server,
    pub owner: Player,

    pub state: LobbyState,
    pub last_state: LobbyState,
}

impl LobbyInfo {
    pub fn new(chat: Chat, player_list: Vec<String>, server: Server, owner: Player) -> Self {
        Self {
            chat,
            player_list,
            server,
            owner,
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Game {
    pub hand: Vec<Card>,
    pub table: Vec<Card>,
    pub triumph: Option<Card>,
}

#[derive(Debug)]
pub struct Chat {
    pub history: Vec<String>,
    pub curr_input: String,
    pub receive: broadcast::Receiver<String>,
    pub send: broadcast::Sender<String>,
}

impl Chat {
    pub fn new(receive: broadcast::Receiver<String>, send: broadcast::Sender<String>) -> Self {
        Self {
            history: Vec::new(),
            curr_input: String::new(),
            receive,
            send,
        }
    }
}

#[derive(Debug)]
pub struct Server {
    pub send: oneshot::Sender<Message>,
    pub recv: oneshot::Receiver<Message>,
}
