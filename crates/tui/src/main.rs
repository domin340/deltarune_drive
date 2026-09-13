mod app;
mod conf;
mod manage_focus;
mod my_widgets;
mod render;

use crate::{
    app::App,
    conf::{Bkp, Conf, extend_bkps_with_fakes},
    manage_focus::{Focus, UiAction},
    my_widgets::input_field::InputAction,
};
use crossterm::event::{self, KeyCode};
use ratatui::DefaultTerminal;
use std::io;

fn main() -> io::Result<()> {
    ratatui::run(run_app)?;
    Ok(())
}

fn handle_input_action(app: &mut App, action: InputAction) {
    match app.focus {
        Focus::BkpName => {
            let res = app.bkp_name_field.handle_action(action);

            if res.confirm {
                app.editing = false;

                let new_name = app.bkp_name_field.to_string();
                app.selected_bkp_mut().map(|bkp| match bkp {
                    Bkp::Unregistered(bkp) => bkp.name = new_name,
                    Bkp::Registered(bkp) => bkp.name = new_name,
                });
            } else if !res.handled {
                app.editing = false;
            }
        }
        _ => {}
    }
}

fn run_app(term: &mut DefaultTerminal) -> io::Result<()> {
    let mut app = create_app();

    'run_app: loop {
        term.draw(|frame| app.ui(frame))?;

        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('q') => break 'run_app,
                _ => {
                    if app.editing {
                        if let Some(action) = InputAction::parse_event(key) {
                            handle_input_action(&mut app, action)
                        }
                    } else if let Some(ui_action) = UiAction::parse(key.code) {
                        app.handle_ui_action(ui_action);
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

fn create_app() -> App {
    let conf = create_conf();
    let mut app = App::from_conf(conf);

    if app.bkps_empty() {
        app.list_item = None;
        app.focus = Focus::ExplorerNew
    } else {
        app.list_item = Some(0.into());
        app.focus = Focus::ExplorerList
    };

    app
}
