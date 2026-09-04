use ratatui::{
    prelude::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{StatefulWidget, Widget},
};

use crate::my_widgets::CornerIndices;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EqPad {
    x: u16,
    y: u16,
}

impl EqPad {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn inner(&self, rect: Rect) -> Rect {
        let x = self.x.min(rect.width / 2);
        let y = self.y.min(rect.height / 2);

        Rect {
            x: rect.x + x,
            y: rect.y + y,
            width: rect.width.saturating_sub(x * 2),
            height: rect.height.saturating_sub(y * 2),
        }
    }
}

impl From<u16> for EqPad {
    fn from(value: u16) -> Self {
        Self::new(value, value)
    }
}

#[derive(Default)]
pub struct ButtonState {
    pub focused: bool,
}

impl ButtonState {
    pub fn set_focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }
}

pub struct Button<'a> {
    padding: EqPad,
    line: Line<'a>,
}

impl Default for Button<'_> {
    fn default() -> Self {
        Self {
            line: "".into(),
            padding: EqPad::from(1),
        }
    }
}

impl<'a> Button<'a> {
    pub fn new(line: impl Into<Line<'a>>) -> Self {
        Self {
            line: line.into(),
            padding: EqPad::from(1),
            ..Default::default()
        }
    }

    pub fn set_padding(mut self, padding: EqPad) -> Self {
        self.padding = padding;
        self
    }
}

impl StatefulWidget for Button<'_> {
    type State = ButtonState;

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let inner = self.padding.inner(area);
        if inner.width < 2 || inner.height < 1 {
            return;
        }

        let CornerIndices {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        } = CornerIndices::from(area);

        let (border, bg, fg) = if state.focused {
            (Color::Gray, Color::DarkGray, Color::White)
        } else {
            (Color::Reset, Color::Reset, Color::Reset)
        };

        let base_style = Style::default().fg(fg).bg(bg);
        buf.set_style(area, base_style);

        let border_style = Style::default().fg(border).bg(bg);
        buf[top_left].set_char('╭').set_style(border_style);
        buf[top_right].set_char('╮').set_style(border_style);
        buf[bottom_left].set_char('╰').set_style(border_style);
        buf[bottom_right].set_char('╯').set_style(border_style);

        if area.width > 2 {
            for x in inner.x..top_right.x {
                buf[(x, top_left.y)].set_char('─').set_style(border_style);
                buf[(x, bottom_left.y)]
                    .set_char('─')
                    .set_style(border_style);
            }
        }

        if area.height > 2 {
            for y in inner.y..bottom_left.y {
                buf[(top_left.x, y)].set_char('│').set_style(border_style);
                buf[(top_right.x, y)].set_char('│').set_style(border_style);
            }
        }

        if inner.width > 0 && inner.height > 0 {
            let line_width = self.line.width() as u16;
            let x = inner
                .x
                .saturating_add(inner.width.saturating_sub(line_width) / 2);
            let y = inner.y + inner.height / 2;
            self.line
                .style(base_style)
                .render(Rect::new(x, y, line_width.min(inner.width), 1), buf);
        }
    }
}
