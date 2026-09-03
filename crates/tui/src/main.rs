mod backups;
mod conf;
mod my_widgets;

use crate::{
    conf::Conf,
    my_widgets::button::{Button, ButtonAction, ButtonState},
};
use crossterm::event::{self, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::Block,
};
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ExplorerItemFocus(usize);

impl ExplorerItemFocus {
    fn item_idx(&self) -> usize {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Focus {
    None,
    Explorer(ExplorerItemFocus),
    RightPanel,
}

impl Focus {
    fn is_explorer(&self) -> bool {
        matches!(self, Focus::Explorer(_))
    }

    fn is_right_panel(&self) -> bool {
        self == &Focus::RightPanel
    }
}

struct State {
    /// stores all the backups and handles the IO inside the local data directory.
    conf: Conf,
    focus: Focus,
}

fn main() -> io::Result<()> {
    ratatui::run(run_app)?;
    Ok(())
}

fn run_app(term: &mut DefaultTerminal) -> io::Result<()> {
    let state = State {
        // conf: Conf::try_load().unwrap_or_default(),
        // temporary line
        conf: Conf::default(),
        focus: Focus::Explorer(ExplorerItemFocus(0)),
    };

    'run_app: loop {
        term.draw(|frame| state.ui(frame))?;

        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('q') => break 'run_app,
                _ => {}
            }
        }
    }

    Ok(())
}

impl State {
    fn ui(&self, frame: &mut Frame) {
        let [explorer_area, right_panel_area] =
            Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)])
                .areas(frame.area());

        let explorer_block = {
            let mut block = Block::bordered().title("Explorer");
            if self.focus.is_explorer() {
                block = block.border_style(Style::default().fg(Color::Blue));
            }

            block
        };

        self.bkp_list(frame, explorer_block.inner(explorer_area));

        let right_panel_block = {
            let mut block = Block::bordered().title("Display");
            if self.focus.is_right_panel() {
                block = block.border_style(Style::default().fg(Color::Blue));
            }

            block
        };

        frame.render_widget(explorer_block, explorer_area);
        frame.render_widget(right_panel_block, right_panel_area);
    }

    fn bkp_list(&self, frame: &mut Frame, area: Rect) {
        let [explorer_list_area, explorer_new_button_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]).areas(area);

        frame.render_stateful_widget(
            // here make the button secondary (find colors for the theme)
            Button::new(Line::from("New").centered()),
            explorer_new_button_area,
            &mut ButtonState::default().set_action(ButtonAction::Focus),
        );
    }
}
