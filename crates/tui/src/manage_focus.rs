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
pub enum MenuFocus {
    #[default]
    Rename,
    Load,
    Clone,
    Delete,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Focus {
    #[default]
    ExplorerList,
    ExplorerNew,
    Menu(MenuFocus),
}

impl Focus {
    pub fn as_menu_index(self) -> Option<usize> {
        if let Focus::Menu(focus) = self {
            Some(focus as usize)
        } else {
            None
        }
    }
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
                Popup::NewBkp { choice } => match BinaryChoiceOutput::parse(*choice, action) {
                    BinaryChoiceOutput::Confirmed(choice) => {
                        if choice == BinaryChoice::Yes {
                            let new_bkp_name = format!("{}", Utc::now().format("%d/%m/%Y %H:%M"));
                            let new_list_idx = self.create_registered_bkp(new_bkp_name);

                            // switch focus to the new backup page
                            self.list_item = Some(new_list_idx.into());
                            self.focus = Focus::ExplorerList;
                        }

                        // otherwise stay where the focus were before.
                        // close popup either way

                        self.popup = None;
                    }
                    BinaryChoiceOutput::FocusOnNo => *choice = BinaryChoice::No,
                    BinaryChoiceOutput::FocusOnYes => *choice = BinaryChoice::Yes,
                    BinaryChoiceOutput::None => {}
                },
                Popup::DeleteBkp { choice } => match BinaryChoiceOutput::parse(*choice, action) {
                    BinaryChoiceOutput::Confirmed(choice) => {
                        if choice == BinaryChoice::Yes {
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
                    BinaryChoiceOutput::FocusOnYes => *choice = BinaryChoice::Yes,
                    BinaryChoiceOutput::FocusOnNo => *choice = BinaryChoice::No,
                    BinaryChoiceOutput::None => {}
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
                UiAction::Enter => Focus::Menu(MenuFocus::default()),
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
                UiAction::Tab => {
                    self.list_item = None;
                    Focus::ExplorerNew
                }
                _ => Focus::ExplorerList,
            },
            Focus::Menu(_) if action == UiAction::Escape => Focus::ExplorerList,
            Focus::Menu(menu) => Focus::Menu(match menu {
                MenuFocus::Rename => match action {
                    UiAction::Down => MenuFocus::Load,
                    _ => MenuFocus::Rename,
                },
                MenuFocus::Load => match action {
                    UiAction::Down => MenuFocus::Clone,
                    UiAction::Up => MenuFocus::Rename,
                    _ => MenuFocus::Load,
                },
                MenuFocus::Clone => match action {
                    UiAction::Down => MenuFocus::Delete,
                    UiAction::Up => MenuFocus::Load,
                    _ => MenuFocus::Clone,
                },
                MenuFocus::Delete => match action {
                    UiAction::Up => MenuFocus::Clone,
                    _ => MenuFocus::Delete,
                },
            }),
        };
    }
}

enum BinaryChoiceOutput {
    Confirmed(BinaryChoice),
    FocusOnYes,
    FocusOnNo,
    None,
}

impl BinaryChoiceOutput {
    pub fn parse(choice: BinaryChoice, action: UiAction) -> Self {
        match action {
            UiAction::Enter => Self::Confirmed(choice),
            UiAction::Y => Self::Confirmed(BinaryChoice::Yes),
            UiAction::N | UiAction::Escape => Self::Confirmed(BinaryChoice::No),
            UiAction::Left => Self::FocusOnYes,
            UiAction::Right => Self::FocusOnNo,
            _ => Self::None,
        }
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
