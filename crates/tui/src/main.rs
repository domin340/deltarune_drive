mod model;
mod my_widgets;

use crate::{
    model::{
        conf::{Conf, extend_bkps_with_fakes},
        state::{Focus, State, UiAction},
    },
    my_widgets::input_field::InputAction,
};
use crossterm::event::{self, KeyCode};
use ratatui::DefaultTerminal;
use std::io;

fn main() -> io::Result<()> {
    ratatui::run(run_app)?;
    Ok(())
}

fn run_app(term: &mut DefaultTerminal) -> io::Result<()> {
    let mut state = create_state();

    'run_app: loop {
        term.draw(|frame| state.ui(frame))?;

        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('q') => break 'run_app,
                _ => {
                    if state.editing {
                        if let Some(action) = InputAction::parse_event(key) {
                            let handled_action = match state.focus {
                                Focus::BkpName => state.bkp_name_field.handle_action(action),
                                Focus::BkpDesc => state.bkp_desc_field.handle_action(action),
                                _ => false,
                            };

                            if !handled_action {
                                state.editing = false;
                            }
                        }
                    } else if let Some(ui_action) = UiAction::parse(key.code) {
                        state.exec_ui_action(ui_action);
                    }
                }
            }
        }
    }

    Ok(())
}

fn create_conf() -> Conf {
    #[cfg(not(debug_assertions))]
    {
        // try loading saved conf from local data directory if found,
        // otherwise create a new one
        Conf::try_load().unwrap_or_default()
    }

    #[cfg(debug_assertions)]
    {
        extend_bkps_with_fakes(Conf::default())
    }
}

fn create_state() -> State {
    let conf = create_conf();
    let mut state = State::from_conf(conf);

    if state.bkps_empty() {
        state.list_item = None;
        state.focus = Focus::ExplorerNew
    } else {
        state.list_item = Some(0.into());
        state.focus = Focus::ExplorerList
    };

    state
}
