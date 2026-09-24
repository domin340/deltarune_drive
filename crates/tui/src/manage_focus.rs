use crate::{
    app::{App, InputPopupModel, Popup},
    my_widgets::popup::BinaryChoice,
};
use crossterm::event::{Event, KeyCode, KeyModifiers};

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

pub struct ConfirmedInput(pub bool);

impl ConfirmedInput {
    pub const fn confirmed_input(self) -> bool {
        self.0
    }
}

fn handle_ui_input_model(model: &mut InputPopupModel, event: UiEvent) -> ConfirmedInput {
    if let Some(choice) = &mut model.submit {
        let UiEvent::Press(e) = event else {
            return ConfirmedInput(false);
        };

        match BinaryChoiceOutput::parse(*choice, e) {
            BinaryChoiceOutput::Confirmed(choice) => {
                if choice == BinaryChoice::Yes {
                    return ConfirmedInput(true);
                } else {
                    // clear the alert popup, go back to input
                    model.submit = None;
                }
            }
            BinaryChoiceOutput::FocusOnNo => *choice = BinaryChoice::No,
            BinaryChoiceOutput::FocusOnYes => *choice = BinaryChoice::Yes,
            BinaryChoiceOutput::None => {}
        }
    } else {
        model.input.handle_action(event);
    }

    ConfirmedInput(false)
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
                    self.popup = Some(Popup::NewBackup(InputPopupModel::default()));
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
                        self.popup = Some(Popup::DeleteBkp(BinaryChoice::Yes));
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
                Popup::NewBackup(model) => {
                    if handle_ui_input_model(model, event).confirmed_input() {
                        let bkp_name = std::mem::take(model.input.buf_mut());
                        let new_list_idx = self.create_registered_bkp(bkp_name);

                        // switch focus to the new backup page
                        self.list_item = Some(new_list_idx.into());
                        self.focus = Focus::ExplorerList;

                        // clear the popup, string buffer data is taken.
                        self.popup = None;
                    }
                }
                Popup::DeleteBkp(choice) => {
                    if let UiEvent::Press(action) = event {
                        match BinaryChoiceOutput::parse(*choice, action) {
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
                        }
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

#[derive(Default)]
enum BinaryChoiceOutput {
    Confirmed(BinaryChoice),
    FocusOnYes,
    FocusOnNo,
    #[default]
    None,
}

impl BinaryChoiceOutput {
    pub fn parse(choice: BinaryChoice, action: UiPress) -> Self {
        match action {
            UiPress::Enter => Self::Confirmed(choice),
            UiPress::Left => Self::FocusOnYes,
            UiPress::Right => Self::FocusOnNo,
            _ => Self::None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiEvent<'paste> {
    Paste(&'paste str),
    Press(UiPress),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiPress {
    Right,
    Left,
    Up,
    Down,
    Enter,
    Escape,
    Tab,
    Char(char),
}

impl<'a> UiEvent<'a> {
    pub fn parse_event(e: &'a Event) -> Option<UiEvent<'a>> {
        Some(match e {
            Event::Paste(paste) => UiEvent::Paste(paste),
            Event::Key(key) if key.is_press() => match key.code {
                KeyCode::Char('v' | 'V') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    // handle paste
                    todo!()
                }
                code => UiEvent::Press(match code {
                    KeyCode::Right => UiPress::Right,
                    KeyCode::Left => UiPress::Left,
                    KeyCode::Up => UiPress::Up,
                    KeyCode::Down => UiPress::Down,
                    KeyCode::Enter => UiPress::Enter,
                    KeyCode::Esc => UiPress::Escape,
                    KeyCode::Tab => UiPress::Tab,
                    KeyCode::Char(c) => UiPress::Char(c),
                    _ => return None,
                }),
            },
            _ => return None,
        })
    }
}
