use crate::model::state::State;
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Focus {
    #[default]
    ExplorerList,
    ExplorerNew,
    BkpName,
    BkpDesc,
    BkpCreated,
    BkpUpdated,
    BkpDelete,
    BkpLoad,
    BkpDuplicate,
    BkpReplace,
}

impl State {
    pub fn list_item_idx(&self) -> Option<usize> {
        self.list_item.map(|item| item.idx())
    }

    fn list_max_idx(&self) -> usize {
        self.conf.bkps().len().saturating_sub(1)
    }

    pub fn exec_ui_action(&mut self, action: UiAction) {
        self.focus = match self.focus {
            Focus::ExplorerNew => match action {
                UiAction::Up if !self.bkps_empty() => {
                    self.list_item = Some(self.list_max_idx().into());
                    Focus::ExplorerList // item above the new button
                }
                UiAction::Tab | UiAction::Down if !self.bkps_empty() => {
                    self.list_item = Some(0.into());
                    Focus::ExplorerList // beginning of the list
                }
                UiAction::Enter => todo!(), // enter popup
                _ => Focus::ExplorerNew,
            },
            Focus::ExplorerList => match action {
                UiAction::Up => {
                    if let Some(item) = self.list_item {
                        if item.idx() == 0 {
                            self.list_item = None;
                            Focus::ExplorerNew
                        } else {
                            self.list_item = Some(item.prev());
                            Focus::ExplorerList
                        }
                    } else {
                        Focus::ExplorerNew
                    }
                }
                UiAction::Down => {
                    if let Some(item) = self.list_item {
                        let max_idx = self.list_max_idx();
                        if item.idx() == max_idx {
                            self.list_item = None;
                            Focus::ExplorerNew
                        } else {
                            self.list_item = Some(item.next().min(max_idx.into()));
                            Focus::ExplorerList
                        }
                    } else {
                        Focus::ExplorerNew
                    }
                }
                UiAction::Enter | UiAction::Right => Focus::BkpName,
                UiAction::Tab => {
                    self.list_item = None;
                    Focus::ExplorerNew
                }
                _ => Focus::ExplorerList,
            },
            Focus::BkpName => match action {
                UiAction::Left => Focus::ExplorerList,
                UiAction::Down | UiAction::Tab => Focus::BkpDesc,
                UiAction::Escape => Focus::ExplorerList,
                _ => Focus::BkpName,
            },
            Focus::BkpDesc => match action {
                UiAction::Left => Focus::ExplorerList,
                UiAction::Down | UiAction::Tab => Focus::BkpCreated,
                UiAction::Up => Focus::BkpName,
                UiAction::Escape => Focus::ExplorerList,
                _ => Focus::BkpDesc,
            },
            Focus::BkpCreated => match action {
                UiAction::Left => Focus::ExplorerList,
                UiAction::Down | UiAction::Tab => Focus::BkpUpdated,
                UiAction::Up => Focus::BkpDesc,
                UiAction::Escape => Focus::ExplorerList,
                _ => Focus::BkpCreated,
            },
            Focus::BkpUpdated => match action {
                UiAction::Left => Focus::ExplorerList,
                UiAction::Down | UiAction::Tab => Focus::BkpDuplicate,
                UiAction::Up => Focus::BkpCreated,
                UiAction::Escape => Focus::ExplorerList,
                _ => Focus::BkpUpdated,
            },
            Focus::BkpDuplicate => match action {
                UiAction::Left => Focus::ExplorerList,
                UiAction::Right | UiAction::Tab => Focus::BkpReplace,
                UiAction::Up => Focus::BkpUpdated,
                UiAction::Escape => Focus::ExplorerList,
                _ => Focus::BkpDuplicate,
            },
            Focus::BkpReplace => match action {
                UiAction::Left => Focus::BkpDuplicate,
                UiAction::Right | UiAction::Tab => Focus::BkpDelete,
                UiAction::Up => Focus::BkpUpdated,
                UiAction::Escape => Focus::ExplorerList,
                _ => Focus::BkpReplace,
            },
            Focus::BkpDelete => match action {
                UiAction::Left => Focus::BkpReplace,
                UiAction::Right | UiAction::Tab => Focus::BkpLoad,
                UiAction::Up => Focus::BkpUpdated,
                UiAction::Escape => Focus::ExplorerList,
                _ => Focus::BkpDelete,
            },
            Focus::BkpLoad => match action {
                UiAction::Left => Focus::BkpDelete,
                UiAction::Tab => Focus::BkpName,
                UiAction::Up => Focus::BkpUpdated,
                UiAction::Escape => Focus::ExplorerList,
                _ => Focus::BkpLoad,
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
