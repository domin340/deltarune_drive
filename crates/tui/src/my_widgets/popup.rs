use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{StatefulWidget, Widget},
};

use crate::my_widgets::button::{Button, ButtonState, EqPad};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinaryChoice {
    #[default]
    Yes,
    No,
}

#[derive(Default, Debug)]
pub struct NewBackupPopup {
    pub pick: BinaryChoice,
}

impl Widget for NewBackupPopup {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        buf.set_style(area, Style::default().bg(Color::DarkGray));

        let [_, label_area, _, buttons_area, _] = Layout::vertical([
            Constraint::Fill(1),   // gap
            Constraint::Length(1), // label
            Constraint::Length(1), // space
            Constraint::Length(3), // buttons
            Constraint::Fill(1),   // gap
        ])
        .areas(area);

        Line::from("Create new backup from files?").render(label_area, buf);

        let [_, yes_btn_area, _, no_btn_area, _] = Layout::horizontal([
            Constraint::Fill(1),   // gap
            Constraint::Length(7), // yes btn
            Constraint::Length(2), // space
            Constraint::Length(7), // no btn
            Constraint::Fill(1),   // gap
        ])
        .areas(buttons_area);

        Button::default().set_padding(EqPad::new(1, 0)).render(
            yes_btn_area,
            buf,
            &mut ButtonState::default().set_focused(self.pick == BinaryChoice::Yes),
        );
        Button::default().set_padding(EqPad::new(1, 0)).render(
            no_btn_area,
            buf,
            &mut ButtonState::default().set_focused(self.pick == BinaryChoice::No),
        );
    }
}

pub enum Popup {
    NewBackup(NewBackupPopup),
}
