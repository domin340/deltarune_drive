use crate::{
    input::{UiEvent, UiPress},
    my_widgets::{
        input_line::InputState,
        popup::{BinaryChoice, InputPopup},
    },
};

#[derive(Default)]
pub struct InputModel {
    pub submit: Option<BinaryChoice>,
    pub input: InputState,
}

impl<'a> From<&'a InputModel> for InputPopup<'a, 'a> {
    fn from(value: &'a InputModel) -> Self {
        Self::from(value.input.input_widget()).with_submit(value.submit)
    }
}

pub enum InputModelCommand {
    None,
    ConfirmInput,
    Close,
}

impl InputModel {
    pub fn handle_ui_event(&mut self, e: UiEvent) -> InputModelCommand {
        if let Some(choice) = &mut self.submit {
            // handling confirm information from user
            if let UiEvent::Press(press) = e
                && let Some(handled_choice) = handle_bchoice_ui(choice, press)
            {
                if handled_choice.confirmed() {
                    return InputModelCommand::ConfirmInput;
                } else {
                    self.submit = None;
                }
            }
        } else {
            // handling input
            match e {
                UiEvent::Press(UiPress::Escape) => return InputModelCommand::Close,
                UiEvent::Press(UiPress::Enter) => self.submit = Some(BinaryChoice::Yes),
                _ => self.input.handle_event(e),
            }
        }

        InputModelCommand::None
    }
}

pub struct ConfirmedChoice(pub bool);

impl ConfirmedChoice {
    pub const fn confirmed(self) -> bool {
        self.0
    }
}

pub fn handle_bchoice_ui(choice: &mut BinaryChoice, press: UiPress) -> Option<ConfirmedChoice> {
    match press {
        UiPress::Enter => return Some(ConfirmedChoice(choice.as_bool())),
        UiPress::Left => *choice = BinaryChoice::Yes,
        UiPress::Right => *choice = BinaryChoice::No,
        _ => {}
    };

    None
}

pub enum Popup {
    NewBackup(InputModel),
    DeleteBackup(BinaryChoice),
    // RenameBackup(InputPopup),
    // LoadBackup(BinaryChoice),
    // DeleteBackup(BinaryChoice),
}
