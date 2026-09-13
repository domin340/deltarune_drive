use ratatui::{layout::Rect, style::Style, text::Line, widgets::Widget};

use crate::my_widgets::button::ButtonState;

#[derive(Default, Debug)]
struct ButtonSimple<'t> {
    line: Line<'t>,
    min_x_pad: u16,
    focus_style: Option<Style>,
}

impl<'t> ButtonSimple<'t> {
    fn new(line: impl Into<Line<'t>>) -> Self {
        Self {
            line: line.into(),
            focus_style: None,
            min_x_pad: 0,
        }
    }

    const fn focus_style(mut self, style: Style) -> Self {
        self.focus_style = Some(style);
        self
    }

    const fn set_min_x_pad(mut self, x_pad: u16) -> Self {
        self.min_x_pad = x_pad;
        self
    }
}

impl ratatui::widgets::StatefulWidget for ButtonSimple<'_> {
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
