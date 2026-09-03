use ratatui::layout::{Position, Rect};

pub mod button;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CornerIndices {
    pub top_left: Position,
    pub top_right: Position,
    pub bottom_left: Position,
    pub bottom_right: Position,
}

impl From<Rect> for CornerIndices {
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
