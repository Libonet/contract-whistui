use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};

use crate::app::{App, Game, GameInfo, Screen};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples

    match &app.current_screen {
        Screen::Game(game_info) => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(4), Constraint::Length(1)])
                .split(frame.area());

            let game_area = chunks[0];
            render_game(game_info, frame, game_area);

            let controls = Line::from("Controls: <- | -> | Enter | T for chat | q to quit")
                .style(Style::default().fg(Color::Cyan).bg(Color::Black))
                .centered();

            frame.render_widget(controls, chunks[1]);
        }
        Screen::Lobby(_) => {}
        Screen::MainMenu => {
            let chunks = Layout::default()
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(frame.area());

            let title = Line::from("Contract Whist")
                .style(Style::default().fg(Color::Cyan).bg(Color::Black))
                .bold()
                .centered();

            let title_area = centered_rect(50, 50, chunks[0]);
            frame.render_widget(title, title_area);

            let button_chunks = Layout::default()
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[1]);

            let create_area = centered_rect(50, 50, button_chunks[0]);
            let create = Paragraph::new(Line::from("Create a lobby (c)").centered()).block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .style(Style::default().fg(Color::Cyan).bg(Color::Black)),
            );

            frame.render_widget(create, create_area);

            let join_area = centered_rect(50, 50, button_chunks[1]);
            let join = Paragraph::new(Line::from("Join a lobby (j)").centered()).block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .style(Style::default().fg(Color::Cyan).bg(Color::Black)),
            );

            frame.render_widget(join, join_area);
        }
    }

    if app.is_exiting {
        //clear popup area
        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(Clear, area);

        let popup_block = Block::default()
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::default().fg(Color::Cyan).bg(Color::Black));

        let exit_text = Text::styled(
            "Would you like to exit? (y/n)",
            Style::default().fg(Color::Red).bold(),
        );
        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        frame.render_widget(exit_paragraph, area);
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

fn render_game(game_info: &GameInfo, frame: &mut Frame, game_area: Rect) {
    let game_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(game_area);

    let game_zone = game_layout[0];
    render_game_zone(game_info, frame, game_zone);

    let chat_score_zone = game_layout[1];
    render_chat_score(game_info, frame, chat_score_zone);
}

fn render_game_zone(game_info: &GameInfo, frame: &mut Frame, game_zone: Rect) {
    let cards_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(game_zone);

    let table_zone = cards_layout[0];
    render_table(&game_info.game, frame, table_zone);

    let hand_zone = cards_layout[1];
    render_hand(&game_info.game, frame, hand_zone);
}

fn render_hand(game: &Game, frame: &mut Frame, hand_zone: Rect) {
    let hand_block = Block::bordered()
        .title("Hand")
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Cyan).bg(Color::Black));

    let inner_zone = hand_block.inner(hand_zone);
    frame.render_widget(hand_block, hand_zone);

    let hand_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(split_equally(game.hand.len()))
        .split(inner_zone);

    for (i, card) in game.hand.iter().enumerate() {
        frame.render_widget((*card).clone(), hand_layout[i]);
    }
}

fn split_equally(amount: usize) -> Vec<Constraint> {
    let mut list = Vec::new();

    let amount: u32 = amount.try_into().unwrap();
    for _i in 0..amount {
        list.push(Constraint::Ratio(1, amount));
    }

    list
}

fn render_table(game: &Game, frame: &mut Frame, table_zone: Rect) {
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

    let table_block = Block::bordered()
        .title("Table")
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Cyan).bg(Color::Black));

    let inner_table = table_block.inner(table_layout[1]);
    frame.render_widget(table_block, table_layout[1]);

    let cards_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(split_equally(game.table.len()))
        .split(inner_table);

    for (i, card) in game.table.iter().enumerate() {
        frame.render_widget((*card).clone(), cards_layout[i]);
    }
}

fn render_chat_score(_game_info: &GameInfo, frame: &mut Frame, chat_score_zone: Rect) {
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
