mod backups;
mod conf;
mod focus_manager;
mod my_widgets;

use crate::{
    backups::Bkp,
    conf::{Conf, extend_bkps_with_fakes},
    focus_manager::{Focus, FocusManager, UiAction},
    my_widgets::button::{Button, ButtonState},
};
use crossterm::event::{self, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Offset, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, List, ListState},
};
use std::io;

fn main() -> io::Result<()> {
    ratatui::run(run_app)?;
    Ok(())
}

fn run_app(term: &mut DefaultTerminal) -> io::Result<()> {
    let mut state = State {
        // conf: Conf::try_load().unwrap_or_default(),
        // temporary line
        conf: extend_bkps_with_fakes(Conf::default()),
        focus: FocusManager::default(),
    };

    'run_app: loop {
        term.draw(|frame| state.ui(frame))?;

        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('q') => break 'run_app,
                _ => {
                    if let Some(ui_action) = UiAction::parse(key.code) {
                        state.focus.exec(ui_action);
                    }
                }
            }
        }
    }

    Ok(())
}

struct State {
    /// stores all the backups and handles the IO inside the local data directory.
    conf: Conf,
    // focus: Focus,
    focus: FocusManager,
}

impl State {
    fn is_explorer_focused(&self) -> bool {
        matches!(self.focus.focus(), Focus::Explorer(_))
    }

    fn is_bkp_page_focused(&self) -> bool {
        matches!(self.focus.focus(), Focus::BkpPage(_))
    }

    fn ui(&self, frame: &mut Frame) {
        let [explorer_area, bkp_page_area] =
            Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)])
                .areas(frame.area());

        // == handle explorer here ==
        let explorer_block = {
            let mut block = Block::bordered().title("Explorer (LEFT)");
            if self.is_explorer_focused() {
                block = block.border_style(Style::default().fg(Color::Blue));
            }

            block
        };

        let [
            explorer_new_button_area,
            explorer_list_label,
            explorer_list_area,
        ] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(explorer_block.inner(explorer_area));

        frame.render_widget(explorer_block, explorer_area);
        frame.render_stateful_widget(
            // here make the button secondary (find colors for the theme)
            Button::new(Line::from("New").centered()),
            explorer_new_button_area,
            &mut ButtonState::default(),
        );
        frame.render_widget(Line::from("[BACKUP LIST]").centered(), explorer_list_label);
        self.bkp_list(explorer_list_area, frame);

        // == handle right panel here ==
        let bkp_page_block = {
            let mut block = Block::bordered().title("Display (RIGHT)");
            if self.is_bkp_page_focused() {
                block = block.border_style(Style::default().fg(Color::Blue));
            }

            block
        };

        frame.render_widget(bkp_page_block, bkp_page_area);
    }

    fn bkp_list(&self, area: Rect, frame: &mut Frame) {
        frame.render_stateful_widget(
            List::new(self.bkp_names()).highlight_style(Modifier::REVERSED),
            area,
            &mut ListState::default().with_selected(self.focus.list_item_idx()),
        );
    }

    fn bkp_names(&self) -> impl Iterator<Item = &str> {
        self.conf.bkps.iter().map(Bkp::name)
    }
}
