use ratatui::{
    prelude::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{StatefulWidget, Widget},
};

use crate::my_widgets::Corners;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
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
            padding: EqPad::default(),
        }
    }
}

impl<'a> Button<'a> {
    pub fn new(line: impl Into<Line<'a>>) -> Self {
        Self {
            line: line.into(),
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

        let Corners {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        } = Corners::from(area);

        let (base_style, border_style) = if state.focused {
            (
                Style::default().fg(Color::White).bg(Color::DarkGray),
                Style::default().fg(Color::Gray).bg(Color::DarkGray),
            )
        } else {
            (Style::default(), Style::default())
        };

        buf.set_style(area, base_style);

        buf[top_left].set_char('╭').set_style(border_style);
        buf[top_right].set_char('╮').set_style(border_style);
        buf[bottom_left].set_char('╰').set_style(border_style);
        buf[bottom_right].set_char('╯').set_style(border_style);

        if area.width > 2 {
            for x in (inner.x + 1)..top_right.x {
                buf[(x, top_left.y)].set_char('─').set_style(border_style);
                buf[(x, bottom_left.y)]
                    .set_char('─')
                    .set_style(border_style);
            }
        }

        if area.height > 2 {
            for y in (inner.y + 1)..bottom_left.y {
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

#[derive(Default, Debug)]
pub struct ButtonSimple<'t> {
    line: Line<'t>,
    min_x_pad: u16,
    focus_style: Option<Style>,
}

impl<'t> ButtonSimple<'t> {
    pub fn new(line: impl Into<Line<'t>>) -> Self {
        Self {
            line: line.into(),
            focus_style: None,
            min_x_pad: 0,
        }
    }

    pub const fn focus_style(mut self, style: Style) -> Self {
        self.focus_style = Some(style);
        self
    }

    pub const fn set_min_x_pad(mut self, x_pad: u16) -> Self {
        self.min_x_pad = x_pad;
        self
    }
}

impl StatefulWidget for ButtonSimple<'_> {
    type State = ButtonState;

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        buf[(area.x, area.y)].set_char('[');
        buf[(area.x + area.width - 1, area.y)].set_char(']');

        if state.focused
            && let Some(focus_style) = self.focus_style
        {
            buf.set_style(Rect { height: 1, ..area }, focus_style);
        }

        let line_area = Rect {
            x: area.x + self.min_x_pad + 1,
            width: area.width - self.min_x_pad - 1,
            ..area
        };

        self.line.render(line_area, buf);
    }
}
