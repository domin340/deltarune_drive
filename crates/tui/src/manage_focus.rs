use crate::{
    app::{App, ExplorerListItem},
    input::{UiEvent, UiPress},
    my_widgets::popup::BinaryChoice,
    popup_models::{InputModel, InputModelCommand, Popup, handle_bchoice_ui},
};

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
    fn predict_focus(&mut self, action: UiPress) -> Focus {
        match self.focus {
            Focus::ExplorerNew => match action {
                UiPress::Up if !self.bkps_empty() => {
                    let last_index = self.last_explorer_list_index();
                    self.list_item = ExplorerListItem::Index(last_index);
                    Focus::ExplorerList // item above the new button
                }
                UiPress::Tab | UiPress::Down if !self.bkps_empty() => {
                    self.list_item = ExplorerListItem::Index(0.into());
                    Focus::ExplorerList // beginning of the list
                }
                UiPress::Enter => {
                    self.popup = Some(Popup::NewBackup(InputModel::default()));
                    Focus::ExplorerNew
                }
                _ => Focus::ExplorerNew,
            },
            Focus::ExplorerList => match action {
                UiPress::Up => match self.list_item {
                    ExplorerListItem::Index(index) => {
                        if index.unwrap() == 0 {
                            self.list_item = ExplorerListItem::None;
                            Focus::ExplorerNew
                        } else {
                            self.list_item = ExplorerListItem::Index(index.prev());
                            Focus::ExplorerList
                        }
                    }
                    ExplorerListItem::None => Focus::ExplorerNew,
                    _ => todo!(),
                },
                UiPress::Enter => Focus::Menu(MenuFocus::default()),
                UiPress::Down => {
                    match self.list_item {
                        ExplorerListItem::Index(index) => {
                            let last_index = self.last_explorer_list_index().unwrap();
                            if index.unwrap() == last_index {
                                self.list_item = ExplorerListItem::None;
                                Focus::ExplorerNew
                            } else {
                                let new_index = index.next().min(last_index.into());
                                self.list_item = ExplorerListItem::Index(new_index);
                                Focus::ExplorerList
                            }
                        }
                        // explorer is empty edge case
                        ExplorerListItem::None => Focus::ExplorerNew,
                        _ => todo!(),
                    }
                }
                UiPress::Tab => {
                    self.list_item = ExplorerListItem::None;
                    Focus::ExplorerNew
                }
                _ => Focus::ExplorerList,
            },
            Focus::Menu(_) if action == UiPress::Escape => Focus::ExplorerList,
            Focus::Menu(menu) => match menu {
                MenuFocus::Rename => match action {
                    UiPress::Down => Focus::Menu(MenuFocus::Load),
                    _ => Focus::Menu(MenuFocus::Rename),
                },
                MenuFocus::Load => match action {
                    UiPress::Down => Focus::Menu(MenuFocus::Clone),
                    UiPress::Up => Focus::Menu(MenuFocus::Rename),
                    _ => Focus::Menu(MenuFocus::Load),
                },
                MenuFocus::Clone => match action {
                    UiPress::Down => Focus::Menu(MenuFocus::Delete),
                    UiPress::Up => Focus::Menu(MenuFocus::Load),
                    _ => Focus::Menu(MenuFocus::Clone),
                },
                MenuFocus::Delete => match action {
                    UiPress::Up => Focus::Menu(MenuFocus::Clone),
                    UiPress::Enter => {
                        self.popup = Some(Popup::DeleteBackup(BinaryChoice::Yes));
                        Focus::ExplorerList
                    }
                    _ => Focus::Menu(MenuFocus::Delete),
                },
            },
        }
    }

    pub fn handle_ui(&mut self, event: UiEvent) {
        if let Some(popup) = &mut self.popup {
            match popup {
                Popup::NewBackup(model) => match model.handle_ui_event(event) {
                    InputModelCommand::Close => self.popup = None,
                    InputModelCommand::ConfirmInput => {
                        let bkp_name = model.input.take_buffer();
                        let new_list_index = self.create_registered_bkp(bkp_name);

                        // switch focus to the new backup page
                        self.list_item = ExplorerListItem::Index(new_list_index.into());
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
                            let ExplorerListItem::Index(index) = self.list_item else {
                                panic!("expected ExplorerListIndex when menu is open");
                            };

                            self.delete_bkp(index.unwrap());

                            self.list_item = if self.conf.bkps.is_empty() {
                                ExplorerListItem::None
                            } else {
                                // move back by 1 bkp
                                let last_index = self.last_explorer_list_index();
                                ExplorerListItem::Index(index.min(last_index))
                            };

                            self.focus = Focus::ExplorerList;
                        }

                        self.popup = None;
                    }
                }
            }

            return;
        };

        if let UiEvent::Press(action) = event {
            self.focus = self.predict_focus(action);
        }
    }
}
