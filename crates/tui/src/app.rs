use crate::manage_focus::{ExplorerListItem, Focus};
use crate::{conf::Conf, my_widgets::input_field::Field};

#[derive(Default)]
pub struct App {
    /// stores all the backups and handles the IO inside the local data directory.
    pub conf: Conf,
    pub focus: Focus,
    pub bkp_name_field: Field,
    pub bkp_desc_field: Field,
    /// NOTE: can be set by [`State::exec_ui_action`] usually by pressing enter
    pub editing: bool,
    pub list_item: Option<ExplorerListItem>,
}

impl App {
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
}
