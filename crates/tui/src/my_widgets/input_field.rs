use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{layout::Position, text::Text};

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

pub struct InputField<T>
where
    for<'text> T: Into<Text<'text>>,
{
    pub text: T,
    pub show_cursor: bool,
}

// pub struct InputField<T: for<'a> Into<Text<'a>>> {
//     text: T,
// }

// impl<T: for<'a> Into<Text<'a>>> InputField<T> {
//     pub fn new(text: T) -> Self {
//         Self { text }
//     }
// }
