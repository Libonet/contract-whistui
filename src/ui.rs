use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Style}, text::Line, widgets::{Block, BorderType, Paragraph, Wrap}, Frame
};

use crate::app::App;

#[allow(dead_code)]
fn render_placeholder(app: &mut App, frame: &mut Frame, game_area: Rect) {
    let placeholder = Paragraph::new(format!(
            "This is a tui template.\n\
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                Counter: {}",
            app.counter
        ))
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .centered();

    let game = placeholder;
    frame.render_widget(game, game_area);
}

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(4),
            Constraint::Length(1),
        ])
        .split(frame.area());
    
    //let game_block = Block::bordered()
    //    .title("Contract Whist")
    //    .title_alignment(Alignment::Center)
    //    .border_type(BorderType::Rounded)
    //    .style(Style::default().fg(Color::Cyan).bg(Color::Black));
    //
    //let game_area = game_block.inner(chunks[0]);
    //frame.render_widget(game_block, chunks[0]);

    //render_placeholder(app, frame, game_area);

    let game_area = chunks[0];
    render_game(app, frame, game_area);

    let controls = Line::from("Controls: <- | -> | Enter | T for chat | q to quit")
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .centered();

    frame.render_widget(controls, chunks[1]);
}

fn render_game(app: &mut App, frame: &mut Frame, game_area: Rect) {
    let game_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(85),
            Constraint::Percentage(15),
        ])
        .split(game_area);

    let game_zone = game_layout[0];
    render_game_zone(app, frame, game_zone);

    let chat_score_zone = game_layout[1];
    render_chat_score(app, frame, chat_score_zone); 
}

fn render_game_zone(app: &mut App, frame: &mut Frame, game_zone: Rect) {
    let cards_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(game_zone);

    let table_zone = cards_layout[0];

    render_table(app, frame, table_zone);

    let hand_zone = cards_layout[1];
    let hand = Paragraph::new(
            "Here goes the hand... if I had one".to_string()
        )
        .centered()
        .block(Block::bordered()
            .title("Hand")
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        );
    frame.render_widget(hand, hand_zone); 
}

fn render_table(_app: &mut App, frame: &mut Frame, table_zone: Rect) {
    let table_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 8),
            Constraint::Ratio(7, 8),
        ])
        .split(table_zone);

    let triumph = Paragraph::new(
            "Here goes the triumph... if I had one".to_string()
        )
        .centered()
        .wrap(Wrap { trim: true })
        .block(Block::bordered()
            .title("Triumph")
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        );
    frame.render_widget(triumph, table_layout[0]);

    let ops_hands = Paragraph::new(
            "Here goes the opponent's hands... if I had them".to_string()
        )
        .centered()
        .block(Block::bordered()
            .title("Opponent Hands")
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        );
        
    frame.render_widget(ops_hands, table_layout[1]);
}

fn render_chat_score(_app: &mut App, frame: &mut Frame, chat_score_zone: Rect) {
    let chat_score_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chat_score_zone);

    let score = Paragraph::new(
            "Here goes the score... if I had one".to_string()
        )
        .centered()
        .wrap(Wrap { trim: true })
        .block(Block::bordered()
            .title("Score")
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        );
    frame.render_widget(score, chat_score_layout[0]);
        
    let chat = Paragraph::new(
            "Here goes the chat... if I had one".to_string()
        )
        .centered()
        .wrap(Wrap { trim: true })
        .block(Block::bordered()
            .title("Chat")
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        );
    frame.render_widget(chat, chat_score_layout[1]);
}
