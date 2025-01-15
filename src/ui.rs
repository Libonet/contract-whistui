use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, Screen};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(4), Constraint::Length(1)])
        .split(frame.area());

    //let game_block = Block::bordered()
    //    .title("Contract Whist")
    //    .title_alignment(Alignment::Center)
    //    .border_type(BorderType::Rounded)
    //    .style(Style::default().fg(Color::Cyan).bg(Color::Black));
    //
    //let game_area = game_block.inner(chunks[0]);
    //frame.render_widget(game_block, chunks[0]);

    let game_area = chunks[0];
    match app.current_screen {
        Screen::Game => {
            render_game(app, frame, game_area);

            let controls = Line::from("Controls: <- | -> | Enter | T for chat | q to quit")
                .style(Style::default().fg(Color::Cyan).bg(Color::Black))
                .centered();

            frame.render_widget(controls, chunks[1]);
        }
        Screen::Lobby => {
        },
        Screen::SearchingForLobby => {
        },
        Screen::MainMenu => {
        },
        Screen::Exiting => {

        },
    }
}

fn render_game(app: &mut App, frame: &mut Frame, game_area: Rect) {
    let game_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(game_area);

    let game_zone = game_layout[0];
    render_game_zone(app, frame, game_zone);

    let chat_score_zone = game_layout[1];
    render_chat_score(app, frame, chat_score_zone);
}

fn render_game_zone(app: &mut App, frame: &mut Frame, game_zone: Rect) {
    let cards_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(game_zone);

    let table_zone = cards_layout[0];

    render_table(app, frame, table_zone);

    let hand_zone = cards_layout[1];
    let hand = Paragraph::new("Here goes the hand... if I had one".to_string())
        .centered()
        .block(
            Block::bordered()
                .title("Hand")
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::Cyan).bg(Color::Black)),
        );
    frame.render_widget(hand, hand_zone);
}

fn render_table(_app: &mut App, frame: &mut Frame, table_zone: Rect) {
    use crate::cards::Card;

    let table_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 8), Constraint::Ratio(7, 8)])
        .split(table_zone);

    let triumph_block = Block::bordered()
        .title("Triumph")
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Cyan).bg(Color::Black));

    let inner_triumph = triumph_block.inner(table_layout[0]);
    frame.render_widget(triumph_block, table_layout[0]);

    let triumph = Card::default();
    frame.render_widget(triumph, inner_triumph);

    let played_cards = Paragraph::new("Here goes the played cards... if I had them".to_string())
        .centered()
        .block(
            Block::bordered()
                .title("Played Cards")
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::Cyan).bg(Color::Black)),
        );

    frame.render_widget(played_cards, table_layout[1]);
}

fn render_chat_score(_app: &mut App, frame: &mut Frame, chat_score_zone: Rect) {
    let chat_score_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chat_score_zone);

    let score = Paragraph::new("Here goes the score... if I had one".to_string())
        .centered()
        .wrap(Wrap { trim: true })
        .block(
            Block::bordered()
                .title("Score")
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::Cyan).bg(Color::Black)),
        );
    frame.render_widget(score, chat_score_layout[0]);

    let chat = Paragraph::new("Here goes the chat... if I had one".to_string())
        .centered()
        .wrap(Wrap { trim: true })
        .block(
            Block::bordered()
                .title("Chat")
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::Cyan).bg(Color::Black)),
        );
    frame.render_widget(chat, chat_score_layout[1]);
}
