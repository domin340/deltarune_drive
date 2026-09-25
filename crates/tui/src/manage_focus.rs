use crate::{
    app::App,
    input::{UiEvent, UiPress},
    my_widgets::popup::BinaryChoice,
    popup_models::{InputModel, InputModelCommand, Popup, handle_bchoice_ui},
};

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

    fn handle_ui_focus(&mut self, action: UiPress) {
        self.focus = match self.focus {
            Focus::ExplorerNew => match action {
                UiPress::Up if !self.bkps_empty() => {
                    self.list_item = Some(self.last_list_item());
                    Focus::ExplorerList // item above the new button
                }
                UiPress::Tab | UiPress::Down if !self.bkps_empty() => {
                    self.list_item = Some(0.into());
                    Focus::ExplorerList // beginning of the list
                }
                UiPress::Enter => {
                    self.popup = Some(Popup::NewBackup(InputModel::default()));
                    Focus::ExplorerNew
                }
                _ => Focus::ExplorerNew,
            },
            Focus::ExplorerList => match action {
                UiPress::Up => {
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
                UiPress::Enter => Focus::Menu(MenuFocus::default()),
                UiPress::Down => {
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
                UiPress::Tab => {
                    self.list_item = None;
                    Focus::ExplorerNew
                }
                _ => Focus::ExplorerList,
            },
            Focus::Menu(_) if action == UiPress::Escape => Focus::ExplorerList,
            Focus::Menu(menu) => Focus::Menu(match menu {
                MenuFocus::Rename => match action {
                    UiPress::Down => MenuFocus::Load,
                    _ => MenuFocus::Rename,
                },
                MenuFocus::Load => match action {
                    UiPress::Down => MenuFocus::Clone,
                    UiPress::Up => MenuFocus::Rename,
                    _ => MenuFocus::Load,
                },
                MenuFocus::Clone => match action {
                    UiPress::Down => MenuFocus::Delete,
                    UiPress::Up => MenuFocus::Load,
                    _ => MenuFocus::Clone,
                },
                MenuFocus::Delete => match action {
                    UiPress::Up => MenuFocus::Clone,
                    UiPress::Enter => {
                        self.popup = Some(Popup::DeleteBackup(BinaryChoice::Yes));
                        MenuFocus::Delete
                    }
                    _ => MenuFocus::Delete,
                },
            }),
        };
    }

    pub fn handle_ui(&mut self, event: UiEvent) {
        if let Some(popup) = &mut self.popup {
            match popup {
                Popup::NewBackup(model) => match model.handle_ui_event(event) {
                    InputModelCommand::Close => self.popup = None,
                    InputModelCommand::ConfirmInput => {
                        let bkp_name = std::mem::take(model.input.buf_mut());
                        let new_list_idx = self.create_registered_bkp(bkp_name);

                        // switch focus to the new backup page
                        self.list_item = Some(new_list_idx.into());
                        self.focus = Focus::ExplorerList;

                        // clear the popup, string buffer data is taken.
                        self.popup = None;
                    }
                    InputModelCommand::None => {}
                },
                Popup::DeleteBackup(choice) => {
                    if let UiEvent::Press(press) = event
                        && let Some(handled_choice) = handle_bchoice_ui(choice, press)
                    {
                        if handled_choice.confirmed() {
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
                }
            }

            return;
        };

        if let UiEvent::Press(press) = event {
            self.handle_ui_focus(press);
        }
    }
}
