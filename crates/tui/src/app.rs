use crate::conf::Conf;
use crate::conf::{Bkp, RegisteredBkp};
use crate::manage_focus::{ExplorerListItem, Focus};
use crate::my_widgets::input_line::InputState;
use crate::my_widgets::popup::{BinaryChoice, InputPopup};

#[derive(Default)]
pub struct InputPopupModel {
    pub submit: Option<BinaryChoice>,
    pub input: InputState,
}

impl<'a> From<&'a InputPopupModel> for InputPopup<'a, 'a> {
    fn from(value: &'a InputPopupModel) -> Self {
        Self::from(value.input.input_widget()).with_submit(value.submit)
    }
}

pub enum Popup {
    NewBackup(InputPopupModel),
    DeleteBkp(BinaryChoice),
    // RenameBackup(InputPopup),
    // LoadBackup(BinaryChoice),
    // DeleteBackup(BinaryChoice),
}

pub struct App {
    /// stores all the backups and handles the IO inside the local data directory.
    pub conf: Conf,
    pub focus: Focus,
    pub editing: bool,
    pub list_item: Option<ExplorerListItem>,
    pub popup: Option<Popup>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            conf: Conf::default(),
            focus: Focus::default(),
            editing: false,
            list_item: None,
            popup: None,
        }
    }

    pub fn from_conf(conf: Conf) -> Self {
        Self {
            conf,
            ..Default::default()
        }
    }

    /// Returns [`true`] when bkps list is emtpy
    pub fn bkps_empty(&self) -> bool {
        self.conf.bkps().len() == 0
    }

    /// Returns index to the backup
    pub(crate) fn create_registered_bkp(&mut self, name: String) -> usize {
        let next_idx = self.conf.bkps.len();

        let registered_bkp = Bkp::Registered(RegisteredBkp::with_name(name));
        self.conf.bkps.push(registered_bkp);

        next_idx
    }

    pub(crate) fn delete_bkp(&mut self, idx: usize) {
        self.conf.bkps.remove(idx);
    }
}
