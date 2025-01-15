use crate::app::{App, AppResult, State, Screen};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.user_state {
        State::Menu => {
            
        },
        State::Playing => {
            match key_event.code {
                // Exit application normal on `ESC` or `q`
                KeyCode::Esc | KeyCode::Char('q') => {
                    app.current_screen = Screen::Exiting;
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
                    app.user_state = State::Chatting;
                },
                // Played a card
                KeyCode::Enter => {
                    app.user_state = State::Idle;
                }
                // Other handlers you could add here.
                _ => {}
            }
        },
        State::Chatting => (),
        State::Idle => (),
    }
    Ok(())
}

