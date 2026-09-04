#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExplorerListItem(pub usize);

impl ExplorerListItem {
    #[inline]
    pub fn idx(self) -> usize {
        self.0
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum ExplorerFocus {
    /// Nothing inside explorer block is selected.
    #[default]
    None,
    List(ExplorerListItem),
    NewButton,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum BkpPageFocus {
    /// First item in the bkp page block
    #[default]
    NameInput,
    DescInput,
    CreatedField,
    UpdatedField,
    DeleteButton,
    DuplicateButton,
    ReplaceButton,
}

#[derive(Default, PartialEq, Eq)]
pub enum Focus {
    #[default]
    None,
    Explorer(ExplorerFocus),
    BkpPage(BkpPageFocus),
}

pub struct FocusManager {
    focus: Focus,
}

impl FocusManager {
    pub fn focus(&self) -> &Focus {
        &self.focus
    }
}
