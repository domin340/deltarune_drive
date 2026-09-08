use crate::manage_focus::{ExplorerListItem, Focus};
use crate::{conf::Conf, my_widgets::input_field::Field};

pub const MAX_NAME_FIELD_LINES: usize = 1;

// FIXME: backup preview shouldn't allocate needlessly for fields when previewing items.
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
            bkp_name_field: Field::default().set_max_lines(MAX_NAME_FIELD_LINES),
            bkp_desc_field: Field::default(),
            editing: false,
            list_item: None,
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
}
