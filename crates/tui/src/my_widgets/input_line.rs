use crate::{UiEvent, input::UiPress};
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::Widget,
};
use std::cell::Cell;

#[derive(Debug, Clone, Copy)]
struct InputDisplayState {
    /// Used to find offset to render input.
    /// Mimics web <input /> behavior>
    ///
    /// ```txt
    /// width = 10
    /// "always say hello to everyone!"
    ///     ^         ^ here is the cursor now
    ///     | first character displayed
    ///
    /// "always say hello to everyone!"
    ///     ^        ^ moved cursor
    ///     | same place, never touched the boundry
    /// ```
    scroll_offset: usize,
    /// Previous area width given during rendering
    area_width: usize,
}

impl Default for InputDisplayState {
    fn default() -> Self {
        Self {
            scroll_offset: 0,
            area_width: usize::MAX,
        }
    }
}

/// Struct that handles input inner buffer and cursor.
/// Additionally handles [`InputAction`].
pub struct InputState {
    s: String,
    /// index bounds: `0..=s.len()`.
    /// Points at a byte not column.
    /// Tells where to insert and what character to delete.
    index: usize,
    /// Points at the character/column.
    cursor: usize,
    /// Tells how many characters (not bytes) `s` buffer should hold inside.
    /// [`usize::MAX`] by default for simplicity.
    max_chars: usize,
    /// Precomputed value of how many characters `s` holds.
    /// Updated every insertion (+1) and deletion (-1).
    chars_len: usize,
    display_state: Cell<InputDisplayState>,
}

impl Default for InputState {
    fn default() -> Self {
        Self::new(String::new())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct InputIndices {
    /// See [`InputState::cursor`]
    cursor: usize,
    /// See [`InputState::index`]
    index: usize,
    /// See [`InputDisplayState::scroll_offset`]
    scroll_offset: usize,
}

impl InputState {
    pub fn new(s: impl Into<String>) -> Self {
        let s = s.into();
        let chars_len = s.chars().count();

        Self {
            s,
            chars_len,
            index: 0,
            cursor: 0,
            display_state: Cell::default(),
            max_chars: usize::MAX,
        }
    }

    fn indices(&self) -> InputIndices {
        InputIndices {
            cursor: self.cursor,
            index: self.index,
            scroll_offset: self.display_state.get().scroll_offset,
        }
    }

    fn update_display_width(&self, new_width: usize) {
        self.display_state.update(|prev| InputDisplayState {
            scroll_offset: prev.scroll_offset,
            area_width: new_width,
        });
    }

    /// Moves index and cursor to the start of the buffer.
    pub fn move_start(&mut self) {
        self.index = 0;
        self.cursor = 0;

        // with cursor being at the start; scroll offset must also be reset.
        let display_state = self.display_state.get_mut();
        display_state.scroll_offset = 0;
    }

    /// *Takes* inner buffer and replaces it with empty one returning the original.
    /// See [`std::mem::take`].
    pub fn take_buffer(&mut self) -> String {
        self.move_start();
        self.chars_len = 0;
        std::mem::take(&mut self.s)
    }

    pub const fn len(&self) -> usize {
        self.s.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.s.is_empty()
    }

    pub fn paste(&mut self, s: &str) {
        let remaining = self.max_chars.saturating_sub(self.chars_len);

        if remaining == 0 {
            return;
        }

        // Don't insert more characters than the limit allows.
        let s = s.chars().take(remaining).collect::<String>();

        if s.is_empty() {
            return;
        }

        let pasted_char_count = s.chars().count();

        self.s.insert_str(self.index, &s);
        self.index += s.len();

        self.chars_len += pasted_char_count;
        self.cursor += pasted_char_count;
        self.update_char_offset();
    }

    pub fn delete(&mut self) {
        if self.index == 0 {
            return;
        }

        let previous = self.s[..self.index]
            .char_indices()
            .next_back()
            .map(|(index, _)| index)
            .expect("index > 0 means there is a previous character");

        self.s.drain(previous..self.index);
        self.index = previous;

        self.chars_len -= 1;
        self.cursor -= 1;
        self.update_char_offset();
    }

    pub fn insert(&mut self, c: char) {
        if self.chars_len >= self.max_chars {
            return;
        }

        self.s.insert(self.index, c);
        self.index += c.len_utf8();

        self.chars_len += 1;
        self.cursor += 1;
        self.update_char_offset();
    }

    const fn update_char_offset(&mut self) {
        let state = self.display_state.get_mut();
        if self.cursor < state.scroll_offset {
            state.scroll_offset = self.cursor;
        } else if self.cursor >= state.scroll_offset + state.area_width {
            state.scroll_offset = self.cursor - state.area_width + 1;
        }
    }

    pub fn move_back(&mut self) {
        self.move_left(1);
    }

    pub fn move_forward(&mut self) {
        self.move_right(1);
    }

    fn move_left(&mut self, amount: usize) {
        let new_cursor = self.cursor.saturating_sub(amount);
        let moved = self.cursor - new_cursor;

        self.cursor = new_cursor;
        self.index = self.s[..self.index]
            .char_indices()
            .nth_back(moved.saturating_sub(1))
            .map(|(index, _)| index)
            .unwrap_or(0);

        self.update_char_offset();
    }

    fn move_right(&mut self, amount: usize) {
        let new_cursor = self.cursor.saturating_add(amount).min(self.chars_len);
        let moved = new_cursor - self.cursor;

        self.cursor = new_cursor;

        self.index = self.s[self.index..]
            .char_indices()
            .nth(moved)
            .map(|(index, _)| self.index + index)
            .unwrap_or(self.s.len());

        self.update_char_offset();
    }

    pub fn handle_event(&mut self, e: UiEvent) {
        match e {
            UiEvent::Paste(s) => self.paste(s),
            UiEvent::Press(press) => match press {
                UiPress::Left => self.move_back(),
                UiPress::Right => self.move_forward(),
                UiPress::Back => self.delete(),
                UiPress::Char(c) => self.insert(c),
                _ => {}
            },
        };
    }
}

impl Widget for &InputState {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let new_width = area.width as usize;
        self.update_display_width(new_width);

        let display_state = self.display_state.get();
        let mut local_cursor_idx = self.cursor - display_state.scroll_offset;

        let scroll_offset = if local_cursor_idx == new_width {
            // now when width = 10 and cursor is 10 (trailing to insert a new character)
            // cursor will be still in area for the user.
            local_cursor_idx -= 1;
            display_state.scroll_offset.saturating_add(1)
        } else {
            display_state.scroll_offset
        };

        // take buffer character count worth of characters bounded by area width
        for (c, local_char_offset) in self.s.chars().skip(scroll_offset).zip(0..area.width) {
            let coord = (area.x.saturating_add(local_char_offset), area.y);
            buf[coord].set_char(c);
        }

        if local_cursor_idx < new_width {
            let cursor_style = Style::default().add_modifier(Modifier::REVERSED);
            let local_cursor_idx = local_cursor_idx as u16;
            buf[(area.x + local_cursor_idx, area.y)].set_style(cursor_style);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::my_widgets::input_line::{InputIndices, InputState};

    /// width = 10
    /// "always say hello to everyone!"
    /// ^          ^ cursor/index = 9
    /// | char offset
    ///
    ///  <- start
    ///
    /// "always say hello to everyone!"
    ///     ^         ^ here is the cursor now
    ///     | first character displayed
    ///
    /// "always say hello to everyone!"
    ///     ^        ^ moved cursor
    ///     | same place, never touched the boundry
    #[test]
    fn display_char_offset_web_behavior() {
        let mut input = InputState::new("always say hello to everyone!");
        input.update_display_width(10);

        input.move_right(10);
        assert_eq!(
            InputIndices {
                index: 10,
                cursor: 10,
                scroll_offset: 1,
            },
            input.indices()
        );

        input.move_right(3);
        assert_eq!(
            InputIndices {
                index: 13,
                cursor: 13,
                scroll_offset: 4,
            },
            input.indices()
        );

        input.paste("HELLO");
        assert_eq!(
            InputIndices {
                index: 18,
                cursor: 18,
                scroll_offset: 9
            },
            input.indices()
        );

        input.move_left(1);
        assert_eq!(
            InputIndices {
                index: 17,
                cursor: 17,
                scroll_offset: 9,
            },
            input.indices()
        );
    }
}
