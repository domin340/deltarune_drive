mod backups;
mod conf;

use crate::conf::Conf;
use crossterm::event::{self, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Style,
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
    let mut state = State {
        // conf: Conf::try_load().unwrap_or_default(),
        // temporary line
        conf: Conf::default(),
        focus: Focus::Explorer(ExplorerItemFocus(0)),
    };

    'run_app: loop {
        term.draw(|frame| draw_ui(&mut state, frame))?;

        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('q') => break 'run_app,
                _ => {}
            }
        }
    }

    Ok(())
}

fn draw_ui(state: &mut State, frame: &mut Frame) {
    let [explorer, right_panel] =
        Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)])
            .areas(frame.area());

    let explorer_block = {
        let mut block = Block::bordered().title("Explorer");
        if state.focus.is_explorer() {
            block = block.border_style(Style::default().fg(styles::FOCUS_COLOR));
        }

        block
    };

    let right_panel_block = {
        let mut block = Block::bordered().title("Display");
        if state.focus.is_right_panel() {
            block = block.border_style(Style::default().fg(styles::FOCUS_COLOR));
        }

        block
    };

    frame.render_widget(explorer_block, explorer);
    frame.render_widget(right_panel_block, right_panel);
}

mod styles {
    use ratatui::style::Color;

    pub const FOCUS_COLOR: Color = Color::Blue;
}
