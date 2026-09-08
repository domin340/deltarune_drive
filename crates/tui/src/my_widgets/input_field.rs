use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Position, Rect},
    style::Modifier,
    text::{Line, Text},
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
            KeyCode::Down => Some(InputAction::MoveDown),
            KeyCode::Up => Some(InputAction::MoveUp),
            KeyCode::Backspace => Some(InputAction::DeleteChar),
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
pub struct Field {
    pub cursor: Cursor,
    pub lines: Vec<String>,
    pub max_lines: Option<usize>,
    pub max_line_len: Option<usize>,
}

impl Field {
    pub fn new(lines: Vec<String>) -> Self {
        Self {
            lines,
            ..Default::default()
        }
    }

    pub const fn set_max_lines(mut self, max_lines: Option<usize>) -> Self {
        self.max_lines = max_lines;
        self
    }

    pub const fn set_max_line_len(mut self, max_line_len: Option<usize>) -> Self {
        self.max_line_len = max_line_len;
        self
    }

    pub fn from_str(s: &str) -> Self {
        Self {
            lines: s.split("\n").map(String::from).collect(),
            ..Default::default()
        }
    }

    pub fn to_input_item(&self) -> FieldItem {
        FieldItem::new(
            self.lines
                .iter()
                .map(|s| Line::raw(s.as_str()))
                .collect::<Text<'_>>(),
        )
        .set_cursor(self.cursor.clone())
    }
}

#[derive(Debug)]
pub struct FieldItem<'b, 't> {
    pub text: Text<'t>,
    pub block: Option<Block<'b>>,
    pub cursor: Option<Cursor>,
}

impl<'b, 't> FieldItem<'b, 't> {
    pub fn new(text: impl Into<Text<'t>>) -> Self {
        Self {
            text: text.into(),
            block: None,
            cursor: None,
        }
    }

    pub fn set_cursor(mut self, cursor: Cursor) -> Self {
        self.cursor = Some(cursor);
        self
    }

    pub fn block(mut self, block: Block<'b>) -> Self {
        self.block = Some(block);
        self
    }
}

impl<'b, 't> Widget for FieldItem<'b, 't> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let text_area = if let Some(block) = self.block {
            let inner = block.inner(area);
            block.render(area, buf);
            inner
        } else {
            area
        };

        if let Some(cursor) = self.cursor {
            let term_cursor_pos = Position {
                x: text_area.x + cursor.x(),
                y: text_area.y + cursor.y(),
            };

            buf.cell_mut(term_cursor_pos)
                .map(|cell| cell.modifier = Modifier::REVERSED);
        }

        self.text.render(text_area, buf);
    }
}
