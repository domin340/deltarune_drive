use ratatui::{
    prelude::Rect,
    style::Style,
    text::Line,
    widgets::{StatefulWidget, Widget},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EqPad {
    x: u16,
    y: u16,
}

impl EqPad {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

impl From<u16> for EqPad {
    fn from(value: u16) -> Self {
        Self::new(value, value)
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum ButtonAction {
    #[default]
    None,
    Focus,
    Press,
}

#[derive(Default)]
pub struct ButtonState {
    pub action: ButtonAction,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum ButtonStyle {
    Ghost,
    #[default]
    Secondary,
}

pub struct Button<'a> {
    press: Option<Style>,
    focus: Option<Style>,
    normal: Option<Style>,
    style: ButtonStyle,
    padding: EqPad,
    line: Line<'a>,
}

impl Default for Button<'_> {
    fn default() -> Self {
        Self {
            press: None,
            focus: None,
            normal: None,
            style: ButtonStyle::default(),
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

    pub fn set_normal_style(mut self, style: Style) -> Self {
        self.normal = Some(style);
        self
    }

    pub fn set_press_style(mut self, style: Style) -> Self {
        self.press = Some(style);
        self
    }

    pub fn set_focus_style(mut self, style: Style) -> Self {
        self.focus = Some(style);
        self
    }

    pub fn set_padding(mut self, padding: EqPad) -> Self {
        self.padding = padding;
        self
    }
}

impl StatefulWidget for Button<'_> {
    type State = ButtonState;

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let button_rect = Rect::new(
            area.x, // start positions
            area.y,
            area.width,
            self.padding.y * 2 + 1,
        );

        if let Some(style) = match state.action {
            ButtonAction::None => self.normal,
            ButtonAction::Focus => self.focus,
            ButtonAction::Press => self.press,
        } {
            buf.set_style(button_rect, style);
        }

        let text_rect = Rect::new(area.x, area.y + self.padding.y, area.width, 1);
        self.line.render(text_rect, buf);
    }
}
