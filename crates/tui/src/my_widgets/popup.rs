use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::Clear,
};

use crate::my_widgets::button::{ButtonSimple, ButtonState};

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

pub fn render_new_bkp_popup(frame: &mut Frame, area: Rect, state: &NewBackupPopup) {
    frame.render_widget(Clear, area);
    frame
        .buffer_mut()
        .set_style(area, Style::default().fg(Color::Reset).bg(Color::Blue));

    let [_, label_area, _, buttons_area, _] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Fill(1),
    ])
    .areas(area);

    let label_text = "create new backup from files?";
    frame.render_widget(Line::from(label_text).centered(), label_area);

    let [_, yes_btn_area, _, no_btn_area, _] = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(16),
        Constraint::Length(2),
        Constraint::Length(16),
        Constraint::Fill(1),
    ])
    .areas(buttons_area);

    let btn_focus_style = Style::default().bg(Color::DarkGray).fg(Color::White);
    frame.render_stateful_widget(
        ButtonSimple::new(Line::from("yes").centered())
            .focus_style(btn_focus_style)
            .set_min_x_pad(1),
        yes_btn_area,
        &mut ButtonState::default().set_focused(state.pick == BinaryChoice::Yes),
    );
    frame.render_stateful_widget(
        ButtonSimple::new(Line::from("no").centered())
            .focus_style(btn_focus_style)
            .set_min_x_pad(1),
        no_btn_area,
        &mut ButtonState::default().set_focused(state.pick == BinaryChoice::No),
    );
}

pub enum Popup {
    NewBackup(NewBackupPopup),
}
