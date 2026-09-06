use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
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
    PasteClipboard,
    Escape,
    Enter,
}

impl InputAction {
    pub fn parse(code: KeyCode, modifiers: KeyModifiers) -> Option<InputAction> {
        match code {
            key if modifiers.is_empty() => match key {
                KeyCode::Backspace => Some(InputAction::DeleteChar),
                KeyCode::Enter => Some(InputAction::Enter),
                KeyCode::Esc => Some(InputAction::Escape),
                KeyCode::Left => Some(InputAction::MoveLeft),
                KeyCode::Right => Some(InputAction::MoveRight),
                KeyCode::Char(c) => Some(InputAction::Insert(c)),
                _ => None,
            },
            KeyCode::Char('v') if modifiers.contains(KeyModifiers::CONTROL) => {
                Some(InputAction::PasteClipboard)
            }
            _ => None,
        }
    }
}

#[derive(Default, Debug)]
pub struct InputFieldState {
    pub local_cursor: Position,
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
                text_area.x + state.local_cursor.x,
                text_area.y + state.local_cursor.y,
            );

            // create cursor
            buf.cell_mut(term_cursor_pos)
                .map(|cell| cell.modifier = Modifier::REVERSED);
        }

        self.text.render(text_area, buf);
    }
}
