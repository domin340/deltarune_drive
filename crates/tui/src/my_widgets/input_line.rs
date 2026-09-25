use crate::{UiEvent, input::UiPress};
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::Widget,
};

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

    /// Creates [`Input`] renderable widget from state.
    /// Borrows current buffer to share with Input.
    pub fn as_input_widget(&self) -> Input<'_> {
        Input::new(&self.s).with_cursor_index(self.index)
    }

    pub const fn len(&self) -> usize {
        self.s.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.s.is_empty()
    }

    pub fn paste_on_index(&mut self, s: &str) {
        let remaining = self.max_chars.saturating_sub(self.chars_len);

        if remaining == 0 {
            return;
        }

        // Don't insert more characters than the limit allows.
        let s = s.chars().take(remaining).collect::<String>();

        if s.is_empty() {
            return;
        }

        self.s.insert_str(self.index, &s);
        self.chars_len += s.chars().count();
        self.index += s.len();
    }

    pub fn delete_on_index(&mut self) {
        if self.index == 0 {
            return;
        }

        let previous = self.s[..self.index]
            .char_indices()
            .next_back()
            .map(|(index, _)| index)
            .expect("index > 0 means there is a previous character");

        self.s.drain(previous..self.index);
        self.chars_len -= 1;
        self.index = previous;
    }

    pub fn insert_on_index(&mut self, c: char) {
        if self.chars_len >= self.max_chars {
            return;
        }

        self.s.insert(self.index, c);
        self.chars_len += 1;
        self.index += c.len_utf8();
    }

    fn move_back(&mut self) {
        if self.index > 0 {
            self.index = self.s[..self.index]
                .char_indices()
                .next_back()
                .map(|(index, _)| index)
                .unwrap_or(0);
        }
    }

    fn move_forward(&mut self) {
        if self.index < self.s.len() {
            let next = self.s[self.index..]
                .chars()
                .next()
                .expect("index < len means there is a character");

            self.index += next.len_utf8();
        }
    }

    pub fn handle_event(&mut self, e: UiEvent) {
        match e {
            UiEvent::Paste(s) => self.paste_on_index(s),
            UiEvent::Press(press) => match press {
                UiPress::Left => self.move_back(),
                UiPress::Right => self.move_forward(),
                UiPress::Back => self.delete_on_index(),
                UiPress::Char(c) => self.insert_on_index(c),
                _ => {}
            },
        };
    }
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
        if area.width == 0 || area.height == 0 {
            return;
        }

        let style = self.style.unwrap_or_default();
        let cursor = self.cursor_index.unwrap_or(usize::MAX);
        let mut x = area.x;

        for (index, c) in self.s.char_indices() {
            if x >= area.right() {
                break;
            }

            let char_width = 1;

            let char_style = if index == cursor {
                style.add_modifier(Modifier::REVERSED)
            } else {
                style
            };

            buf.set_string(x, area.y, c.to_string(), char_style);

            x += char_width;
        }

        // Cursor at the end of the string.
        if cursor == self.s.len() && x < area.right() {
            buf.set_string(x, area.y, " ", style.add_modifier(Modifier::REVERSED));
        }
    }
}
