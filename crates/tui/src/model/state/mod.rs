mod manage_focus;
mod render;

use crate::{
    model::conf::Conf,
    my_widgets::input_field::{Cursor, Field, InputAction},
};
pub use manage_focus::{ExplorerListItem, Focus, UiAction};

/// String divided into lines.
#[derive(Default)]
pub struct LinesString(pub Vec<String>);

impl LinesString {
    pub fn from_vec(v: Vec<String>) -> Self {
        Self(v)
    }

    pub fn from_iter(v: impl Iterator<Item = impl Into<String>>) -> Self {
        Self::from_vec(v.map(Into::into).collect())
    }

    pub fn lines(&self) -> impl Iterator<Item = &str> {
        self.0.iter().map(String::as_str)
    }

    pub const fn len(&self) -> usize {
        self.0.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get_line(&self, line: usize) -> Option<&str> {
        self.0.get(line).map(String::as_str)
    }
}

impl ToString for LinesString {
    fn to_string(&self) -> String {
        self.0.join("\n")
    }
}

impl From<String> for LinesString {
    fn from(value: String) -> Self {
        Self(value.split('\n').map(String::from).collect())
    }
}

#[derive(Default)]
pub struct State {
    /// stores all the backups and handles the IO inside the local data directory.
    pub conf: Conf,
    pub focus: Focus,
    pub bkp_name_field: Field,
    pub bkp_desc_field: Field,
    /// NOTE: can be set by [`State::exec_ui_action`] usually by pressing enter
    pub editing: bool,
    pub list_item: Option<ExplorerListItem>,
}

impl State {
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
