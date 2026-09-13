use crate::{
    app::{App, Popup},
    my_widgets::popup::BinaryChoice,
};
use chrono::Utc;
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

    fn last_list_item(&self) -> ExplorerListItem {
        self.conf.bkps().len().saturating_sub(1).into()
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
                Popup::NewBkp { choice } => match process_binary_choice_popup(choice, action) {
                    HandledBinaryChoice::Confirmed { pick } => {
                        if pick {
                            let new_bkp_name = format!("{}", Utc::now().format("%d/%m/%Y %H:%M"));
                            let new_list_idx = self.create_bkp(new_bkp_name);

                            // switch focus to the new backup page
                            self.list_item = Some(new_list_idx.into());
                            self.focus = Focus::BkpName;
                        }

                        // otherwise stay where the focus were before.
                        // close popup either way

                        self.popup = None;
                    }
                    HandledBinaryChoice::FocusOnNo => *choice = BinaryChoice::No,
                    HandledBinaryChoice::FocusOnYes => *choice = BinaryChoice::Yes,
                    HandledBinaryChoice::None => {}
                },
                Popup::DeleteBkp { choice } => match process_binary_choice_popup(choice, action) {
                    HandledBinaryChoice::Confirmed { pick } => {
                        if pick {
                            let current_list_item = self.list_item.unwrap();
                            self.delete_bkp(current_list_item.idx());

                            self.list_item = if self.conf.bkps.is_empty() {
                                None
                            } else {
                                Some(current_list_item.min(self.last_list_item())) // move back by 1 bkp
                            };
                        }

                        self.popup = None;
                    }
                    HandledBinaryChoice::FocusOnYes => *choice = BinaryChoice::Yes,
                    HandledBinaryChoice::FocusOnNo => *choice = BinaryChoice::No,
                    HandledBinaryChoice::None => {}
                },
            };

            return;
        }

        self.focus = match self.focus {
            Focus::ExplorerNew => match action {
                UiAction::Up if !self.bkps_empty() => {
                    self.list_item = Some(self.last_list_item());
                    Focus::ExplorerList // item above the new button
                }
                UiAction::Tab | UiAction::Down if !self.bkps_empty() => {
                    self.list_item = Some(0.into());
                    Focus::ExplorerList // beginning of the list
                }
                UiAction::Enter => {
                    let popup = Popup::NewBkp {
                        choice: BinaryChoice::default(),
                    };
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
                UiAction::D => {
                    self.popup = Some(Popup::DeleteBkp {
                        choice: BinaryChoice::default(),
                    });

                    Focus::ExplorerList
                }
                UiAction::Down => {
                    if let Some(item) = self.list_item {
                        let last_idx = self.last_list_item().idx();
                        if item.idx() == last_idx {
                            self.list_item = None;
                            Focus::ExplorerNew
                        } else {
                            self.list_item = Some(item.next().min(last_idx.into()));
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
                    let bkp_name = self.selected_bkp().name();
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

enum HandledBinaryChoice {
    Confirmed { pick: bool },
    FocusOnYes,
    FocusOnNo,
    None,
}

fn process_binary_choice_popup(choice: &mut BinaryChoice, action: UiAction) -> HandledBinaryChoice {
    match action {
        UiAction::Enter => {
            let pick = choice == &BinaryChoice::Yes;
            HandledBinaryChoice::Confirmed { pick }
        }
        UiAction::Y => HandledBinaryChoice::Confirmed { pick: true },
        UiAction::N => HandledBinaryChoice::Confirmed { pick: false },
        UiAction::Left => HandledBinaryChoice::FocusOnYes,
        UiAction::Right => HandledBinaryChoice::FocusOnNo,
        _ => HandledBinaryChoice::None,
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
    /// Delete for short
    D,
    /// Yes for short
    Y,
    /// No for short
    N,
}

impl UiAction {
    pub fn parse(code: KeyCode) -> Option<Self> {
        Some(match code {
            KeyCode::Right => Self::Right,
            KeyCode::Left => Self::Left,
            KeyCode::Up => Self::Up,
            KeyCode::Down => Self::Down,
            KeyCode::Enter => Self::Enter,
            KeyCode::Esc => Self::Escape,
            KeyCode::Tab => Self::Tab,
            KeyCode::Char(c) => match c {
                'd' => Self::D,
                'y' => Self::Y,
                'n' => Self::N,
                _ => return None,
            },
            _ => return None,
        })
    }
}
