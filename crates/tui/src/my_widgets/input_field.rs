use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Position, Rect},
    style::Modifier,
    text::Text,
    widgets::{Block, StatefulWidget, Widget},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum InputAction {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Insert(char),
    DeleteChar,
    Escape,
    Enter,
}

impl InputAction {
    pub fn parse_event(e: KeyEvent) -> Option<InputAction> {
        if !e.modifiers.is_empty() {
            return None;
        }

        match e.code {
            KeyCode::Enter => Some(InputAction::Enter),
            KeyCode::Esc => Some(InputAction::Escape),
            KeyCode::Left => Some(InputAction::MoveLeft),
            KeyCode::Right => Some(InputAction::MoveRight),
            KeyCode::Char(c) => Some(InputAction::Insert(c)),
            _ => None,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cursor {
    x: i32,
    y: i32,
}

// Methods must ensure that the x and y positions are in u16 bounds
impl Cursor {
    pub const fn new(x: u16, y: u16) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }

    pub const fn x(&self) -> u16 {
        self.x as u16
    }

    pub const fn y(&self) -> u16 {
        self.y as u16
    }

    pub const fn moved_by(&self, x: i32, y: i32) -> Self {
        let mut cursor = Cursor {
            x: self.x.saturating_add(x),
            y: self.y.saturating_add(y),
        };

        if cursor.x < 0 {
            cursor.x = 0;
        }

        if cursor.y < 0 {
            cursor.y = 0;
        }

        cursor
    }
}

impl From<(i32, i32)> for Cursor {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(
            x.try_into().unwrap_or_default(),
            y.try_into().unwrap_or_default(),
        )
    }
}

impl From<Position> for Cursor {
    fn from(value: Position) -> Self {
        Self::new(value.x, value.y)
    }
}

#[derive(Default, Debug)]
pub struct InputFieldState {
    /// Local cursor, relative to input field
    pub cursor: Cursor,
}

#[derive(Debug)]
pub struct InputField<'block, 'text> {
    pub text: Text<'text>,
    pub block: Option<Block<'block>>,
    pub show_cursor: bool,
}

impl<'block, 'text> InputField<'block, 'text> {
    pub fn new(text: impl Into<Text<'text>>) -> Self {
        Self {
            text: text.into(),
            block: None,
            show_cursor: true,
        }
    }

    pub fn show_cursor(mut self, show: bool) -> Self {
        self.show_cursor = show;
        self
    }

    pub fn block(mut self, block: Block<'block>) -> Self {
        self.block = Some(block);
        self
    }
}

impl<'block, 'text> StatefulWidget for InputField<'block, 'text> {
    type State = InputFieldState;

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let text_area = if let Some(block) = self.block {
            let inner = block.inner(area);
            block.render(area, buf);
            inner
        } else {
            area
        };

        if self.show_cursor {
            let term_cursor_pos = (
                text_area.x + state.cursor.x(),
                text_area.y + state.cursor.y(),
            );

            // create cursor
            buf.cell_mut(term_cursor_pos)
                .map(|cell| cell.modifier = Modifier::REVERSED);
        }

        self.text.render(text_area, buf);
    }
}
