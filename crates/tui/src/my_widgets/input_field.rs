use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Position, Rect},
    style::Modifier,
    text::{Line, Text},
    widgets::{Block, Widget},
};

/// See [`Self::parse_event`]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum InputAction {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Insert(char),
    DeleteChar,
    Escape,
    Newline,
    Confirm,
}

impl InputAction {
    pub fn parse_event(e: KeyEvent) -> Option<InputAction> {
        match e.code {
            KeyCode::Enter if e.modifiers.contains(KeyModifiers::SHIFT) => {
                Some(InputAction::Newline)
            }
            code if e.modifiers.is_empty() => match code {
                KeyCode::Enter => Some(InputAction::Confirm),
                KeyCode::Esc => Some(InputAction::Escape),
                KeyCode::Left => Some(InputAction::MoveLeft),
                KeyCode::Right => Some(InputAction::MoveRight),
                KeyCode::Down => Some(InputAction::MoveDown),
                KeyCode::Up => Some(InputAction::MoveUp),
                KeyCode::Backspace => Some(InputAction::DeleteChar),
                KeyCode::Char(c) => Some(InputAction::Insert(c)),
                _ => None,
            },
            _ => None,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cursor {
    x: i32,
    y: i32,
}

// Methods must ensure that the x and y positions are in u16 bounds
impl Cursor {
    pub const fn new(x: u16, y: u16) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }

    pub const fn x(&self) -> u16 {
        self.x as u16
    }

    pub const fn y(&self) -> u16 {
        self.y as u16
    }

    pub const fn moved_by(&self, x: i32, y: i32) -> Self {
        let mut cursor = Cursor {
            x: self.x.saturating_add(x),
            y: self.y.saturating_add(y),
        };

        if cursor.x < 0 {
            cursor.x = 0;
        }

        if cursor.y < 0 {
            cursor.y = 0;
        }

        cursor
    }

    const fn raw_x(&self) -> i32 {
        self.x
    }

    const fn raw_y(&self) -> i32 {
        self.y
    }
}

impl From<Position> for Cursor {
    fn from(value: Position) -> Self {
        Self::new(value.x, value.y)
    }
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HandledInputAction {
    pub confirm: bool,
    pub handled: bool,
}

impl HandledInputAction {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_handled(mut self, handled: bool) -> Self {
        self.handled = handled;
        self
    }

    pub fn set_confirm(mut self, confirm: bool) -> Self {
        self.confirm = confirm;
        self
    }
}

#[derive(Debug)]
pub struct Field {
    pub cursor: Cursor,
    pub lines: Vec<String>,
    pub max_lines: usize,
    pub max_line_len: usize,
    /// Cached char count per line, kept in sync with `lines`.
    /// Index-aligned with `lines` — line_lens[i] == lines[i].chars().count().
    line_lens: Vec<usize>,
}

impl Default for Field {
    fn default() -> Self {
        let lines = vec![String::new()];
        Self::new(lines)
    }
}

impl ToString for Field {
    fn to_string(&self) -> String {
        self.lines.join("\n")
    }
}

fn line_lens(v: &Vec<String>) -> impl Iterator<Item = usize> {
    v.iter().map(|s| s.chars().count())
}

impl Field {
    pub fn new(lines: Vec<String>) -> Self {
        if lines.is_empty() {
            panic!("lines mustn't be empty! at least a single element is required");
        }

        Self {
            line_lens: line_lens(&lines).collect(),
            lines,
            cursor: Cursor::default(),
            max_lines: usize::MAX,
            max_line_len: usize::MAX,
        }
    }

    pub fn set_lines(&mut self, lines: Vec<String>) {
        self.lines = lines;
        self.line_lens = line_lens(&self.lines).collect();
    }

    pub const fn set_max_lines(mut self, max_lines: usize) -> Self {
        self.max_lines = max_lines;
        self
    }

    pub const fn set_max_line_len(mut self, max_line_len: usize) -> Self {
        self.max_line_len = max_line_len;
        self
    }

    pub fn from_str(s: &str) -> Self {
        Self::new(s.split('\n').map(String::from).collect())
    }

    pub fn handle_action(&mut self, action: InputAction) -> HandledInputAction {
        let mut handled_action = true;
        let mut confirm = false;

        match action {
            InputAction::MoveLeft => self.move_cursor(-1, 0),
            InputAction::MoveRight => self.move_cursor(1, 0),
            InputAction::MoveUp => self.move_cursor(0, -1),
            InputAction::MoveDown => self.move_cursor(0, 1),
            InputAction::Insert(c) => self.insert_char(c),
            InputAction::DeleteChar => self.delete_char(),
            InputAction::Newline => self.insert_newline(),
            InputAction::Confirm => confirm = true,
            InputAction::Escape => handled_action = false,
        };

        HandledInputAction::default()
            .set_confirm(confirm)
            .set_handled(handled_action)
    }

    pub fn to_input_item(&self) -> FieldItem<'_, '_> {
        FieldItem::new(
            self.lines
                .iter()
                .map(|s| Line::raw(s.as_str()))
                .collect::<Text<'_>>(),
        )
    }

    pub fn current_line(&self) -> &str {
        let idx = self.cursor.raw_y() as usize;
        self.lines[idx].as_str()
    }

    pub fn line(&self, idx: usize) -> Option<&str> {
        self.lines.get(idx).map(String::as_str)
    }

    pub fn current_line_mut(&mut self) -> &mut String {
        let idx = self.cursor.raw_y() as usize;
        &mut self.lines[idx]
    }

    pub fn line_mut(&mut self, idx: usize) -> Option<&mut String> {
        self.lines.get_mut(idx)
    }

    pub const fn lines_count(&self) -> usize {
        self.lines.len()
    }

    /// Public getters for the pre-computed per-line char counts.
    pub fn line_lens(&self) -> &[usize] {
        &self.line_lens
    }

    pub fn line_len(&self, idx: usize) -> usize {
        self.line_lens[idx]
    }

    /// Cached char count of the line the cursor is currently on — O(1).
    pub fn current_line_len(&self) -> usize {
        self.line_lens[self.cursor.raw_y() as usize]
    }

    pub fn cursor_limit_y(&self) -> usize {
        self.max_line_len.min(self.lines_count())
    }

    pub fn cursor_limit_x(&self) -> usize {
        self.max_line_len.min(self.current_line_len())
    }

    pub fn cursor_to(&mut self, c: Cursor) {
        let x_bound = self.cursor_limit_x().min(u16::MAX as usize) as u16;
        let y_bound = self.cursor_limit_y().min(u16::MAX as usize) as u16;
        self.cursor = Cursor::new(c.x().min(x_bound), c.y().min(y_bound))
    }

    fn move_cursor(&mut self, dx: i32, dy: i32) {
        let target = self.cursor.moved_by(dx, dy);
        self.cursor_to(target);
    }

    /// Byte offset in the current line for the cursor's char-column position.
    fn byte_offset_in_line(&self) -> usize {
        let col = self.cursor.raw_x() as usize;
        self.current_line()
            .char_indices()
            .nth(col)
            .map(|(i, _)| i)
            .unwrap_or_else(|| self.current_line().len())
    }

    pub fn insert_char(&mut self, c: char) {
        let y = self.cursor.raw_y() as usize;

        if self.max_line_len != 0 && self.line_lens[y] >= self.max_line_len {
            return;
        }

        let offset = self.byte_offset_in_line();
        self.lines[y].insert(offset, c);
        self.line_lens[y] += 1;

        let next = self.cursor.moved_by(1, 0);
        self.cursor_to(next);
    }

    pub fn delete_char(&mut self) {
        let col = self.cursor.raw_x() as usize;
        let y = self.cursor.raw_y() as usize;

        if col == 0 {
            // At start of line: merge with previous line, if any.
            if y == 0 {
                return;
            }
            let current = self.lines.remove(y);
            let current_len = self.line_lens.remove(y);

            let prev_len = self.line_lens[y - 1];
            self.lines[y - 1].push_str(&current);
            self.line_lens[y - 1] += current_len;

            let target = Cursor::new(prev_len as u16, (y - 1) as u16);
            self.cursor_to(target);
            return;
        }

        let offset = self.byte_offset_in_line();
        let prev_offset = self.lines[y]
            .char_indices()
            .nth(col - 1)
            .map(|(i, _)| i)
            .unwrap_or(0);
        self.lines[y].replace_range(prev_offset..offset, "");
        self.line_lens[y] -= 1;

        let target = self.cursor.moved_by(-1, 0);
        self.cursor_to(target);
    }

    pub fn insert_newline(&mut self) {
        if self.max_lines != 0 && self.lines_count() >= self.max_lines {
            return;
        }

        let y = self.cursor.raw_y() as usize;
        let offset = self.byte_offset_in_line();

        let current = self.lines[y].clone();
        let (left, right) = current.split_at(offset);

        let right_len = right.chars().count();
        let left_len = self.line_lens[y] - right_len;

        self.lines[y] = left.to_string();
        self.line_lens[y] = left_len;

        self.lines.insert(y + 1, right.to_string());
        self.line_lens.insert(y + 1, right_len);

        let target = Cursor::new(0, (y + 1) as u16);
        self.cursor_to(target);
    }
}

#[derive(Debug)]
pub struct FieldItem<'b, 't> {
    pub text: Text<'t>,
    pub block: Option<Block<'b>>,
    pub cursor: Option<Cursor>,
}

impl<'t> From<&'t str> for FieldItem<'_, 't> {
    fn from(value: &'t str) -> Self {
        Self::from_str(value)
    }
}

impl<'b, 't> FieldItem<'b, 't> {
    pub fn from_str(s: &'t str) -> Self {
        Self {
            text: Text::from(s),
            block: None,
            cursor: None,
        }
    }

    pub fn new(text: impl Into<Text<'t>>) -> Self {
        Self {
            text: text.into(),
            block: None,
            cursor: None,
        }
    }

    pub fn set_cursor(mut self, cursor: Option<Cursor>) -> Self {
        self.cursor = cursor;
        self
    }

    pub fn block(mut self, block: Block<'b>) -> Self {
        self.block = Some(block);
        self
    }
}

impl<'b, 't> Widget for FieldItem<'b, 't> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let text_area = if let Some(block) = self.block {
            let inner = block.inner(area);
            block.render(area, buf);
            inner
        } else {
            area
        };

        if let Some(cursor) = self.cursor {
            let term_cursor_pos = Position {
                x: text_area.x + cursor.x(),
                y: text_area.y + cursor.y(),
            };

            buf.cell_mut(term_cursor_pos)
                .map(|cell| cell.modifier = Modifier::REVERSED);
        }

        self.text.render(text_area, buf);
    }
}
