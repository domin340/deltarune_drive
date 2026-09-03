use ratatui::layout::{Position, Rect};

pub mod button;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Corners {
    pub top_left: Position,
    pub top_right: Position,
    pub bottom_left: Position,
    pub bottom_right: Position,
}

impl From<Rect> for Corners {
    fn from(rect: Rect) -> Self {
        Self {
            top_left: Position::new(rect.x, rect.y),
            top_right: Position::new(rect.x + rect.width, rect.y),
            bottom_left: Position::new(rect.x, rect.y + rect.height),
            bottom_right: Position::new(rect.x + rect.width, rect.y + rect.height),
        }
    }
}
