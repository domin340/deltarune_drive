use crossterm::event::KeyCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExplorerListItem(pub usize);

impl ExplorerListItem {
    #[inline]
    pub fn idx(self) -> usize {
        self.0
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum ExplorerFocus {
    /// Nothing inside explorer block is selected.
    #[default]
    None,
    List(ExplorerListItem),
    NewButton,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum BkpPageFocus {
    /// First item in the bkp page block
    #[default]
    NameInput,
    DescInput,
    CreatedField,
    UpdatedField,
    DeleteButton,
    DuplicateButton,
    ReplaceButton,
}

#[derive(Default, PartialEq, Eq)]
pub enum Focus {
    #[default]
    None,
    Explorer(ExplorerFocus),
    BkpPage(BkpPageFocus),
}

pub struct FocusManager {
    focus: Focus,
}

impl FocusManager {
    pub fn focus(&self) -> &Focus {
        &self.focus
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UiAction {
    Right,
    Left,
    Up,
    Down,
    Enter,
    Escape,
    Tab,
}

impl UiAction {
    pub fn parse(code: KeyCode) -> Option<Self> {
        match code {
            KeyCode::Right => Some(UiAction::Right),
            KeyCode::Left => Some(UiAction::Left),
            KeyCode::Up => Some(UiAction::Up),
            KeyCode::Down => Some(UiAction::Down),
            KeyCode::Enter => Some(UiAction::Enter),
            KeyCode::Esc => Some(UiAction::Escape),
            KeyCode::Tab => Some(UiAction::Tab),
            _ => None,
        }
    }
}
