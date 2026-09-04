mod manage_focus;
mod render;

use crate::model::conf::Conf;
pub use manage_focus::{BkpPageFocus, ExplorerFocus, ExplorerListItem, Focus, UiAction};

#[derive(Default)]
pub struct State {
    /// stores all the backups and handles the IO inside the local data directory.
    pub conf: Conf,
    pub focus: Focus,
    pub list_item: Option<ExplorerListItem>,
}

impl State {
    pub fn from_conf(conf: Conf) -> Self {
        Self {
            conf,
            ..Default::default()
        }
    }
}
