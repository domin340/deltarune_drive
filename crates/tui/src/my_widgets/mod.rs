use ratatui::layout::{Position, Rect};

pub mod button;
pub mod input_field;
pub mod input_line;
pub mod menu;
pub mod popup;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Corners {
    pub top_left: Position,
    pub top_right: Position,
    pub bottom_left: Position,
    pub bottom_right: Position,
}

impl From<Rect> for Corners {
    fn from(rect: Rect) -> Self {
        let x_right_idx = rect.x + rect.width - 1;
        let y_bottom_idx = rect.y + rect.height - 1;
        Self {
            top_left: Position::new(rect.x, rect.y),
            top_right: Position::new(x_right_idx, rect.y),
            bottom_left: Position::new(rect.x, y_bottom_idx),
            bottom_right: Position::new(x_right_idx, y_bottom_idx),
        }
    }
}

pub mod styles {
    use ratatui::style::{Color, Style};

    pub const MENU: Style = Style::new().bg(Color::Reset);

    pub const POPUP: Style = Style::new().bg(Color::Blue);

    pub const ACTIVE_BLOCK: Style = Style::new().fg(Color::Blue);

    pub const FOCUSED: Style = Style::new().bg(Color::DarkGray).fg(Color::White);
}
