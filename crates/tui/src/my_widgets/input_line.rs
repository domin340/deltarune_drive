use crate::UiEvent;
use ratatui::{layout::Rect, style::Style, widgets::Widget};

/// Struct that handles input inner buffer and cursor.
/// Additionally handles [`InputAction`].
pub struct InputState {
    s: String,
    /// index bounds: `0..=s.len()`.
    /// Points at a byte not column.
    /// Tells where to insert and what character to delete.
    index: usize,
    /// Tells how many characters (not bytes) `s` buffer should hold inside.
    /// [`usize::MAX`] by default for simplicity.
    max_chars: usize,
    /// Precomputed value of how many characters `s` holds.
    /// Updated every insertion (+1) and deletion (-1).
    chars_len: usize,
}

impl Default for InputState {
    fn default() -> Self {
        Self::new(String::new())
    }
}

impl InputState {
    pub fn new(s: impl Into<String>) -> Self {
        let s = s.into();
        let chars_len = s.chars().count();

        Self {
            s,
            chars_len,
            index: 0,
            max_chars: usize::MAX,
        }
    }

    pub(crate) const fn buf_mut(&mut self) -> &mut String {
        &mut self.s
    }

    pub const fn cursor_index(&self) -> usize {
        self.index
    }

    /// Creates [`Input`] renderable widget from state.
    /// Borrows current buffer to share with Input.
    pub fn input_widget(&self) -> Input<'_> {
        Input::new(&self.s).with_cursor_index(self.index)
    }

    pub const fn len(&self) -> usize {
        self.s.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.s.is_empty()
    }

    pub fn move_to(&mut self, index: usize) {
        self.index = index.min(self.len())
    }

    pub fn handle_action(&mut self, action: UiEvent) {}

    /*
    todo:
        - pub fn paste(&mut self, s: &str) // pastes s at index
        - pub fn insert(&mut self, char) // inserts character at index
        - pub fn delete(&mut self) // deletes character at index - 1, make sure index isn't 0
        - pub fn handle_action(&mut self, action: InputAction) // handles action with helper methods creates
    */
}

pub struct Input<'line> {
    s: &'line str,
    cursor_index: Option<usize>,
    style: Option<Style>,
}

impl<'line> Input<'line> {
    pub const fn new(s: &'line str) -> Self {
        Self {
            s,
            style: None,
            cursor_index: None,
        }
    }

    pub const fn else_with_placeholder(mut self, s: &'line str) -> Self {
        if self.s.is_empty() {
            self.s = s;
        }

        self
    }

    pub const fn with_cursor_index(mut self, cursor_index: usize) -> Self {
        self.cursor_index = Some(cursor_index);
        self
    }

    pub const fn with_style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }
}

impl Widget for Input<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
    }
}
