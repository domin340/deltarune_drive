mod model;
mod my_widgets;

use crate::{
    model::conf::{Conf, extend_bkps_with_fakes},
    model::state::{Focus, State, UiAction},
};
use crossterm::event::{self, KeyCode};
use ratatui::DefaultTerminal;
use std::io;

fn main() -> io::Result<()> {
    ratatui::run(run_app)?;
    Ok(())
}

fn run_app(term: &mut DefaultTerminal) -> io::Result<()> {
    let mut state = State::from_conf(extend_bkps_with_fakes(Conf::default()));

    'run_app: loop {
        term.draw(|frame| state.ui(frame))?;

        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('q') => break 'run_app,
                _ => {
                    if let Some(ui_action) = UiAction::parse(key.code) {
                        state.exec(ui_action);
                    }
                }
            }
        }
    }

    Ok(())
}
