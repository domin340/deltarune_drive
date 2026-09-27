use crate::conf::Conf;
use crate::conf::{Bkp, RegisteredBkp};
use crate::manage_focus::Focus;
use crate::my_widgets::input_line::InputState;
use crate::my_widgets::popup::BinaryChoice;
use crate::popup_models::Popup;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExplorerListIndex(usize);

impl ExplorerListIndex {
    pub fn unwrap(self) -> usize {
        self.0
    }

    pub fn next(self) -> Self {
        self.unwrap().saturating_add(1).into()
    }

    pub fn prev(self) -> Self {
        self.unwrap().saturating_sub(1).into()
    }
}

impl From<usize> for ExplorerListIndex {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

pub enum CloningAction {
    Pointing(ExplorerListIndex),
    New,
}

#[derive(Default)]
pub enum ExplorerListItem {
    /// Nothing is selected
    #[default]
    None,
    Cloning {
        index: ExplorerListIndex,
        action: CloningAction,
        confirmation: Option<BinaryChoice>,
    },
    Renaming {
        index: ExplorerListIndex,
        input_state: InputState,
        confirmation: Option<BinaryChoice>,
    },
    New {
        input_state: InputState,
        confirmation: Option<BinaryChoice>,
    },
    Index(ExplorerListIndex),
}

impl ExplorerListItem {
    pub fn as_index(&self) -> Option<ExplorerListIndex> {
        if let Self::Index(index) = self {
            Some(index.clone())
        } else {
            None
        }
    }
}

pub struct App {
    /// stores all the backups and handles the IO inside the local data directory.
    pub conf: Conf,
    pub focus: Focus,
    pub list_item: ExplorerListItem,
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
            list_item: ExplorerListItem::None,
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
    pub(crate) fn bkps_empty(&self) -> bool {
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

    pub(crate) fn get_bkp(&self, idx: usize) -> Option<&Bkp> {
        self.conf.bkps.get(idx)
    }

    pub(crate) fn last_explorer_list_index(&self) -> ExplorerListIndex {
        self.conf.bkps().len().saturating_sub(1).into()
    }
}
