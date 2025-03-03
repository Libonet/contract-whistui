use crate::{app::{App, AppResult, GameState, LobbyState, Popup, Screen}, server};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if let Some(popup) = &mut app.popups {
        match popup {
            Popup::Exiting => match key_event.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    app.quit();
                }
                KeyCode::Char('n') | KeyCode::Char('N') => {
                    app.popups = None;
                }
                _ => {}
            },
            Popup::TextBox(_text) => {}
        }

        return Ok(());
    }

    match &mut app.current_screen {
        Screen::Game(game_info) => {
            match game_info.state {
                GameState::Playing => {
                    match key_event.code {
                        // Exit application normal on `ESC` or `q`
                        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                            app.popups = Some(Popup::Exiting);
                        }
                        // Exit application on `Ctrl-C`
                        KeyCode::Char('c') | KeyCode::Char('C') => {
                            if key_event.modifiers == KeyModifiers::CONTROL {
                                app.quit();
                            }
                        }
                        KeyCode::Right => {}
                        KeyCode::Left => {}
                        KeyCode::Char('t') | KeyCode::Char('T') => {
                            game_info.change_state_to(GameState::Chatting);
                        }
                        // Played a card
                        KeyCode::Enter => {
                            game_info.state = GameState::Idle;
                        }
                        // Other handlers you could add here.
                        _ => {}
                    }
                }
                GameState::Chatting => match key_event.code {
                    KeyCode::Esc => {
                        game_info.toggle_state();
                    }
                    KeyCode::Char(ch) => {
                        game_info.chat.curr_input.push(ch);
                    }
                    KeyCode::Backspace => {
                        game_info.chat.curr_input.pop();
                    }
                    KeyCode::Enter => {
                        todo!("Send message to all players and update own history");
                    }
                    _ => {}
                },
                GameState::Idle => {
                    match key_event.code {
                        // Exit application normal on `ESC` or `q`
                        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                            app.popups = Some(Popup::Exiting);
                        }
                        // Exit application on `Ctrl-C`
                        KeyCode::Char('c') | KeyCode::Char('C') => {
                            if key_event.modifiers == KeyModifiers::CONTROL {
                                app.quit();
                            }
                        }
                        KeyCode::Char('t') | KeyCode::Char('T') => {
                            game_info.change_state_to(GameState::Chatting);
                        }
                        // Other handlers you could add here.
                        _ => {}
                    }
                }
            }
        }
        Screen::MainMenu => match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                app.popups = Some(Popup::Exiting);
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                // Create a lobby
                server::create_server();

                server::join_server();
            }
            KeyCode::Char('j') | KeyCode::Char('J') => {
                server::join_server();
            }
            _ => {}
        },
        Screen::Lobby(lobby_info) => match lobby_info.state {
            LobbyState::Readying => match key_event.code {
                KeyCode::Char('s') | KeyCode::Char('S') => {
                    if app.player == lobby_info.owner {
                        todo!("Start game")
                    }
                }
                KeyCode::Char('t') | KeyCode::Char('T') => {}
                _ => {}
            },
            LobbyState::Chatting => match key_event.code {
                KeyCode::Esc => {
                    lobby_info.toggle_state();
                }
                KeyCode::Char(ch) => {
                    lobby_info.chat.curr_input.push(ch);
                }
                KeyCode::Backspace => {
                    lobby_info.chat.curr_input.pop();
                }
                KeyCode::Enter => {
                    todo!("Send message to all players and update own history");
                }
                _ => {}
            },
        },
    }

    Ok(())
}
