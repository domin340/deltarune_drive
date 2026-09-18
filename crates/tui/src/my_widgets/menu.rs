use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{StatefulWidget, Widget},
};

#[derive(Default)]
pub struct MenuState {
    selected: Option<usize>,
}

impl MenuState {
    pub const fn new() -> Self {
        Self { selected: None }
    }

    pub fn with_selected(&mut self, selected: usize) -> &mut Self {
        self.selected = Some(selected);
        self
    }
}

pub struct MenuItem<'text, 'key> {
    text: Span<'text>,
    key: Option<Span<'key>>,
}

impl<'text, 'key> MenuItem<'text, 'key> {
    pub fn new(text: impl Into<Span<'text>>) -> Self {
        Self {
            text: text.into(),
            key: None,
        }
    }

    pub fn with_key(mut self, key: impl Into<Span<'key>>) -> Self {
        self.key = Some(key.into());
        self
    }
}

pub struct Menu<'text, 'key> {
    items: Vec<MenuItem<'text, 'key>>,
    selected_style: Option<Style>,
    /// Width of the gap between text and key if present inside [`MenuItem`]
    menu_item_gap: usize,
}

impl Default for Menu<'_, '_> {
    fn default() -> Self {
        Self {
            items: vec![],
            selected_style: None,
            menu_item_gap: 2,
        }
    }
}

impl<'text, 'key> Menu<'text, 'key> {
    pub fn new<Item: Into<MenuItem<'text, 'key>>>(items: impl Iterator<Item = Item>) -> Self {
        Self {
            items: items.map(Into::into).collect(),
            ..Default::default()
        }
    }

    pub const fn with_menu_item_gap(mut self, gap: usize) -> Self {
        self.menu_item_gap = gap;
        self
    }

    pub const fn with_selected_style(mut self, style: Style) -> Self {
        self.selected_style = Some(style);
        self
    }
}

impl StatefulWidget for Menu<'_, '_> {
    type State = MenuState;

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let iter_start = state
            .selected
            .unwrap_or_default()
            .saturating_sub(area.height as usize - 1);

        if area.width == 0 || area.height == 0 {
            // no point in running the loop when either is zero.
            // additionally may cause issues inside the loop
            return;
        }

        ratatui::widgets::Clear.render(area, buf);
        buf.set_style(area, Style::default().fg(Color::Reset).bg(Color::Reset));

        for ((mut item, item_idx), y_cord) in self
            .items
            .into_iter()
            .skip(iter_start)
            .zip(iter_start..)
            .zip(area.y..(area.y + area.height))
        {
            let item_area = Rect {
                x: area.x,
                y: y_cord,
                width: area.width,
                height: 1,
            };

            if let Some(selected_idx) = state.selected
                && item_idx == selected_idx
                && let Some(selected_style) = self.selected_style
            {
                buf.set_style(item_area, selected_style);
            }

            let w = item_area.width as usize;
            let key_w = item.key.as_ref().map(|key| key.width()).unwrap_or_default();

            if item.text.width() + key_w + self.menu_item_gap > w {
                let shortened_len = w - self.menu_item_gap - key_w;
                let t = item.text.content.to_mut();

                let (col_idx, _) = t
                    .char_indices()
                    .nth(shortened_len.saturating_sub(1))
                    .unwrap(); // guarded by the (width == 0) if statement above.

                unsafe {
                    t.as_bytes_mut()[col_idx] = b'.';
                }

                t.truncate(shortened_len);
            }

            if let Some(key) = item.key {
                let key_area = Rect {
                    x: ((area.x + area.width) as usize - key.width()).min(u16::MAX as usize) as u16,
                    ..item_area
                };

                key.render(key_area, buf);
            }

            item.text.render(item_area, buf);
        }
    }
}
