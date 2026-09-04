mod manage_focus;
mod render;

use crate::conf::Conf;
pub use manage_focus::{BkpPageFocus, ExplorerFocus, ExplorerListItem, Focus, UiAction};

pub struct State {
    /// stores all the backups and handles the IO inside the local data directory.
    pub conf: Conf,
    pub focus: Focus,
    pub list_item: Option<ExplorerListItem>,
}
