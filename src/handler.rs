use crate::app::{App, AppResult, GameState, Screen};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if app.is_exiting {
        match key_event.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                app.quit();
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                app.is_exiting = false;
            }
            _ => {}
        }

        return Ok(());
    }

    match &mut app.current_screen {
        Screen::Game(game_info) => {
            match game_info.game.state {
                GameState::Playing => {
                    match key_event.code {
                        // Exit application normal on `ESC` or `q`
                        KeyCode::Esc | KeyCode::Char('q') => {
                            app.is_exiting = true;
                        }
                        // Exit application on `Ctrl-C`
                        KeyCode::Char('c') | KeyCode::Char('C') => {
                            if key_event.modifiers == KeyModifiers::CONTROL {
                                app.quit();
                            }
                        }
                        KeyCode::Right => {
                        }
                        KeyCode::Left => {
                        }
                        KeyCode::Char('t') | KeyCode::Char('T') => {
                            game_info.change_state_to(GameState::Chatting);
                        },
                        // Played a card
                        KeyCode::Enter => {
                            game_info.game.state = GameState::Idle;
                        }
                        // Other handlers you could add here.
                        _ => {}
                    }
                },
                GameState::Chatting => {
                    match key_event.code {
                        KeyCode::Esc => {
                            game_info.change_state_to(game_info.game.last_state);
                        }
                        KeyCode::Char(ch) => {
                            game_info.chat.curr_input.push(ch);
                        }
                        KeyCode::Backspace => {
                            game_info.chat.curr_input.pop();
                        }
                        _ => {}
                    }
                },
                GameState::Idle => {
                    match key_event.code {
                        // Exit application normal on `ESC` or `q`
                        KeyCode::Esc | KeyCode::Char('q') => {
                            app.is_exiting = true;
                        }
                        // Exit application on `Ctrl-C`
                        KeyCode::Char('c') | KeyCode::Char('C') => {
                            if key_event.modifiers == KeyModifiers::CONTROL {
                                app.quit();
                            }
                        }
                        KeyCode::Char('t') | KeyCode::Char('T') => {
                            game_info.change_state_to(GameState::Chatting);
                        },
                        // Other handlers you could add here.
                        _ => {}
                    }
                }, 
            }
        }
        Screen::MainMenu => {
            match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                    app.is_exiting = true;
                },
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    todo!("Create a lobby");
                },
                KeyCode::Char('j') | KeyCode::Char('J') => {
                    todo!("Join a lobby");
                },
                _ => {}
            }
        }
        _ => (),
    }

    Ok(())
}

