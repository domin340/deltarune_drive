use crate::{
    app::App,
    my_widgets::popup::{BinaryChoice, NewBackupPopup, Popup},
};
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

impl App {
    pub fn list_item_idx(&self) -> Option<usize> {
        self.list_item.map(|item| item.idx())
    }

    fn list_max_idx(&self) -> usize {
        self.conf.bkps().len().saturating_sub(1)
    }

    pub fn handle_ui_action(&mut self, action: UiAction) {
        /*
        Idea:

        handle_ui_key(&App, KeyEvent) -> UiAction {
            { popup, foucs } = app
            UiKey::parse(KeyEvent) ui_key {
                if popup && ui_key == Enter && popup.pick == Yes {
                    return UiAction::NewBkp {
                        name: popup.name_field.to_string()
                    }
                }

                if editing {
                    return InputAction::parse(KeyEvent) action {
                        UiAction::Input(action)
                    }
                }

                match focus {
                    Focus::BkpName { editing: false } && ui_key == Enter => {
                        return UiAction::FocusOn(Focus::BkpName { editing: true })
                    }
                    ...
                }

                ...
            }

            ...
        }
        */

        if let Some(popup) = &mut self.popup {
            match popup {
                Popup::NewBackup(popup) => match action {
                    UiAction::Left => popup.pick = BinaryChoice::Yes,
                    UiAction::Right => popup.pick = BinaryChoice::No,
                    UiAction::Escape => self.popup = None,
                    _ => {}
                },
            };

            return;
        }

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
                UiAction::Enter => {
                    let popup = Popup::NewBackup(NewBackupPopup::default());
                    self.popup = Some(popup);
                    Focus::ExplorerNew
                }
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
                UiAction::Enter => {
                    self.editing = true;
                    let bkp_name = self.selected_bkp().unwrap().name();
                    self.bkp_name_field.set_line(0, bkp_name.to_string());
                    Focus::BkpName
                }
                _ => Focus::BkpName,
            },
            Focus::BkpDesc => match action {
                UiAction::Left => Focus::ExplorerList,
                UiAction::Down | UiAction::Tab => Focus::BkpCreated,
                UiAction::Up => Focus::BkpName,
                UiAction::Escape => Focus::ExplorerList,
                UiAction::Enter => {
                    self.editing = true;
                    Focus::BkpDesc
                }
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
            KeyCode::Right => Some(Self::Right),
            KeyCode::Left => Some(Self::Left),
            KeyCode::Up => Some(Self::Up),
            KeyCode::Down => Some(Self::Down),
            KeyCode::Enter => Some(Self::Enter),
            KeyCode::Esc => Some(Self::Escape),
            KeyCode::Tab => Some(Self::Tab),
            _ => None,
        }
    }
}
