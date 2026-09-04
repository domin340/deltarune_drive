use crossterm::event::KeyCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExplorerListItem(pub usize);

impl ExplorerListItem {
    pub fn idx(self) -> usize {
        self.0
    }

    pub fn next(self) -> ExplorerListItem {
        self.idx().saturating_add(1).into()
    }

    pub fn prev(self) -> ExplorerListItem {
        self.idx().saturating_sub(1).into()
    }
}

impl From<usize> for ExplorerListItem {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum ExplorerFocus {
    /// Nothing inside explorer block is selected.
    #[default]
    None,
    List,
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

#[derive(Default)]
pub struct FocusManager {
    focus: Focus,
    list_item: Option<ExplorerListItem>,
    list_max_idx: Option<usize>,
}

impl FocusManager {
    pub fn set_list_max_idx(&mut self, max: Option<usize>) {
        self.list_max_idx = max;
    }

    pub fn focus(&self) -> &Focus {
        &self.focus
    }

    pub fn list_item(&self) -> Option<ExplorerListItem> {
        self.list_item
    }

    pub fn list_item_idx(&self) -> Option<usize> {
        self.list_item().map(|item| item.idx())
    }

    pub fn exec(&mut self, action: UiAction) {
        let focus = std::mem::take(&mut self.focus);
        self.focus = match focus {
            Focus::None => match action {
                UiAction::Left | UiAction::Enter | UiAction::Tab => {
                    Focus::Explorer(ExplorerFocus::None)
                }
                UiAction::Right => Focus::BkpPage(BkpPageFocus::NameInput),
                _ => Focus::None,
            },
            Focus::Explorer(focus) => match focus {
                ExplorerFocus::None => match action {
                    UiAction::Tab | UiAction::Enter | UiAction::Down => {
                        Focus::Explorer(ExplorerFocus::NewButton)
                    }
                    UiAction::Right => Focus::BkpPage(BkpPageFocus::NameInput),
                    UiAction::Escape => Focus::None,
                    _ => Focus::Explorer(focus), // do nothing
                },
                ExplorerFocus::NewButton => match action {
                    UiAction::Down | UiAction::Tab => {
                        self.list_item = Some(0.into());
                        Focus::Explorer(ExplorerFocus::List)
                    }
                    UiAction::Escape => Focus::None,
                    UiAction::Enter => todo!(), // enter popup
                    _ => Focus::Explorer(ExplorerFocus::NewButton),
                },
                ExplorerFocus::List => match action {
                    UiAction::Up => {
                        let item = self.list_item.unwrap();
                        if item.idx() == 0 {
                            // new button is above it
                            Focus::Explorer(ExplorerFocus::NewButton)
                        } else {
                            self.list_item = Some(item.prev());
                            Focus::Explorer(ExplorerFocus::List)
                        }
                    }
                    UiAction::Down => {
                        let item = self.list_item.unwrap();
                        self.list_item = Some(if let Some(list_max_idx) = self.list_max_idx {
                            item.next().min(list_max_idx.into()) // clamp the idx
                        } else {
                            item.next()
                        });
                        Focus::Explorer(ExplorerFocus::List)
                    }
                    UiAction::Tab => Focus::Explorer(ExplorerFocus::NewButton),
                    UiAction::Enter => Focus::BkpPage(BkpPageFocus::NameInput),
                    UiAction::Escape => Focus::Explorer(ExplorerFocus::None),
                    _ => Focus::Explorer(ExplorerFocus::List),
                },
            },
            Focus::BkpPage(focus) => match focus {
                BkpPageFocus::NameInput => match action {
                    UiAction::Left => Focus::Explorer(ExplorerFocus::None),
                    UiAction::Down | UiAction::Tab => Focus::BkpPage(BkpPageFocus::DescInput),
                    UiAction::Escape => Focus::None,
                    _ => Focus::BkpPage(BkpPageFocus::NameInput),
                },
                BkpPageFocus::DescInput => match action {
                    UiAction::Left => Focus::Explorer(ExplorerFocus::None),
                    UiAction::Down | UiAction::Tab => Focus::BkpPage(BkpPageFocus::CreatedField),
                    UiAction::Up => Focus::BkpPage(BkpPageFocus::NameInput),
                    UiAction::Escape => Focus::None,
                    _ => Focus::BkpPage(BkpPageFocus::DescInput),
                },
                BkpPageFocus::CreatedField => match action {
                    UiAction::Left => Focus::Explorer(ExplorerFocus::None),
                    UiAction::Down | UiAction::Tab => Focus::BkpPage(BkpPageFocus::UpdatedField),
                    UiAction::Up => Focus::BkpPage(BkpPageFocus::DescInput),
                    UiAction::Escape => Focus::None,
                    _ => Focus::BkpPage(BkpPageFocus::CreatedField),
                },
                BkpPageFocus::UpdatedField => match action {
                    UiAction::Left => Focus::Explorer(ExplorerFocus::None),
                    UiAction::Down | UiAction::Tab => Focus::BkpPage(BkpPageFocus::DuplicateButton),
                    UiAction::Up => Focus::BkpPage(BkpPageFocus::CreatedField),
                    UiAction::Escape => Focus::None,
                    _ => Focus::BkpPage(BkpPageFocus::UpdatedField),
                },
                BkpPageFocus::DuplicateButton => match action {
                    UiAction::Left => Focus::Explorer(ExplorerFocus::None),
                    UiAction::Right | UiAction::Tab => Focus::BkpPage(BkpPageFocus::ReplaceButton),
                    UiAction::Up => Focus::BkpPage(BkpPageFocus::UpdatedField),
                    UiAction::Escape => Focus::None,
                    _ => Focus::BkpPage(BkpPageFocus::DuplicateButton),
                },
                BkpPageFocus::ReplaceButton => match action {
                    UiAction::Left => Focus::BkpPage(BkpPageFocus::DuplicateButton),
                    UiAction::Right | UiAction::Tab => Focus::BkpPage(BkpPageFocus::DeleteButton),
                    UiAction::Up => Focus::BkpPage(BkpPageFocus::UpdatedField),
                    UiAction::Escape => Focus::None,
                    _ => Focus::BkpPage(BkpPageFocus::ReplaceButton),
                },
                BkpPageFocus::DeleteButton => match action {
                    UiAction::Left => Focus::BkpPage(BkpPageFocus::ReplaceButton),
                    UiAction::Tab => Focus::BkpPage(BkpPageFocus::NameInput),
                    UiAction::Up => Focus::BkpPage(BkpPageFocus::UpdatedField),
                    UiAction::Escape => Focus::None,
                    _ => Focus::BkpPage(BkpPageFocus::DeleteButton),
                },
            },
        };
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
