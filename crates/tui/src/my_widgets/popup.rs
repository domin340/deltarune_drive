use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Clear, StatefulWidget, Widget},
};

use crate::my_widgets::{
    button::{ButtonSimple, ButtonState},
    input_line::Input,
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinaryChoice {
    #[default]
    Yes,
    No,
}

impl BinaryChoice {
    pub const fn as_bool(self) -> bool {
        match self {
            BinaryChoice::No => false,
            BinaryChoice::Yes => true,
        }
    }
}

#[derive(Default, Debug)]
pub struct BinaryChoicePopup<'t> {
    question_line: Line<'t>,
}

impl<'t> BinaryChoicePopup<'t> {
    pub fn new(line: impl Into<Line<'t>>) -> Self {
        Self {
            question_line: line.into(),
        }
    }
}

impl StatefulWidget for BinaryChoicePopup<'_> {
    type State = BinaryChoice;

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        Clear.render(area, buf);
        buf.set_style(area, Style::default().fg(Color::Reset).bg(Color::Blue));

        let [_, label_area, _, buttons_area, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .areas(area);

        self.question_line.render(label_area, buf);

        let [_, yes_btn_area, _, no_btn_area, _] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(16),
            Constraint::Length(2),
            Constraint::Length(16),
            Constraint::Fill(1),
        ])
        .areas(buttons_area);

        let btn_focus_style = Style::default().bg(Color::DarkGray).fg(Color::White);
        let yes_picked = state == &BinaryChoice::Yes;

        ButtonSimple::new(Line::from("yes").centered())
            .focus_style(btn_focus_style)
            .set_min_x_pad(1)
            .render(
                yes_btn_area,
                buf,
                &mut ButtonState::default().set_focused(yes_picked),
            );

        ButtonSimple::new(Line::from("no").centered())
            .focus_style(btn_focus_style)
            .set_min_x_pad(1)
            .render(
                no_btn_area,
                buf,
                &mut ButtonState::default().set_focused(!yes_picked),
            );
    }
}

pub struct InputPopup<'a, 'line> {
    question: &'line str,
    input: Input<'a>,
    submit: Option<BinaryChoice>,
}

impl<'a> From<Input<'a>> for InputPopup<'a, '_> {
    fn from(input: Input<'a>) -> Self {
        Self {
            input,
            question: "",
            submit: None,
        }
    }
}

impl<'a, 'line> InputPopup<'a, 'line> {
    pub const fn with_submit(mut self, submit: Option<BinaryChoice>) -> Self {
        self.submit = submit;
        self
    }

    pub const fn with_question(mut self, s: &'line str) -> Self {
        self.question = s;
        self
    }
}

impl Widget for InputPopup<'_, '_> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        if let Some(mut submit) = self.submit {
            let line = Line::raw(self.question);
            BinaryChoicePopup::new(line).render(area, buf, &mut submit);
        } else {
            let input_area = area.centered_vertically(Constraint::Length(1));
            self.input.render(input_area, buf);
        }
    }
}
