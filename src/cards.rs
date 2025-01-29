use std::{cmp::min, fmt::Display};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Widget},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Clubs => write!(f, "♣️"),
            Suit::Diamonds => write!(f, "♦️"),
            Suit::Hearts => write!(f, "♥️"),
            Suit::Spades => write!(f, "♠️"),
        }
    }
}

impl From<Suit> for String {
    fn from(value: Suit) -> Self {
        match value {
            Suit::Clubs => "Club".to_string(),
            Suit::Diamonds => "Diamonds".to_string(),
            Suit::Hearts => "Hearts".to_string(),
            Suit::Spades => "Spades".to_string(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum Value {
    Num(u8),
    J,
    Q,
    K,
    A,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Num(n) => write!(f, "{n}"),
            face => write!(f, "{face:?}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Card {
    value: Value,
    suit: Suit,
    player: Option<String>,
    selected: bool,
}

pub enum CardError {
    InvalidValue,
}

impl Card {
    pub fn new(value: Value, suit: Suit, player: Option<String>, selected: bool) -> Result<Self, CardError> {
        if let Value::Num(n) = value {
            if !(2..=10).contains(&n) {
                return Err(CardError::InvalidValue);
            }
        }

        Ok(Self { value, suit, player, selected })
    } 

    pub fn value(mut self, value: Value) -> Self {
        self.value = value;
        self
    }

    pub fn suit(mut self, suit: Suit) -> Self {
        self.suit = suit;
        self
    }
    
    pub fn player(mut self, player: String) -> Self {
        self.player = Some(player);
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn toggle_selected(mut self) -> Self {
        self.selected = !self.selected;
        self
    }
}

impl Default for Card {
    fn default() -> Self {
        Self { value: Value::A, suit: Suit::Diamonds, player: None, selected: false }
    }
}

fn find_ideal_area(width: u16, height: u16) -> (u16,u16) {
    let ideal = (3,3);
    
    let w_ratio = width / ideal.0;
    let h_ratio = height / ideal.1;

    let ratio = min(w_ratio, h_ratio);

    (ideal.0 * ratio, ideal.1 * ratio)
}

impl Widget for Card {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let area = area.intersection(buf.area);
        if area.is_empty() {
            return;
        }

        let (width, height) = find_ideal_area(area.width, area.height);
        let offset = ((area.width - width) / 2, (area.height - height) / 2);
        let area = Rect { width, height, x: area.x + offset.0, y: area.y + offset.1 };

        let mut card_border = Block::bordered()
            .style(Style::default().fg(Color::Black).bg(Color::White));

        if let Some(name) = self.player {
            card_border = card_border.title(name)
        }

        if self.selected {
            card_border = card_border.border_style(Style::new().yellow())
        }

        let layout_area = card_border.inner(area);
        card_border.render(area, buf);

        let card_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(3),
                Constraint::Length(1),
            ])
            .split(layout_area);

        let upper = Line::from(self.value.to_string()).left_aligned();
        let suit = Line::from(self.suit.to_string()).centered();
        let lower = Line::from(self.value.to_string()).right_aligned();

        let suit_area = center(
            card_layout[1], 
            Constraint::Length(2),
            Constraint::Length(1),
        );

        upper.render(card_layout[0], buf);
        suit.render(suit_area, buf);
        lower.render(card_layout[2], buf);
    }
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
