mod app;
mod conf;
mod input;
mod manage_focus;
mod my_widgets;
mod popup_models;
mod render;

use crate::{
    app::App,
    conf::{Conf, extend_bkps_with_fakes},
    input::UiEvent,
    manage_focus::Focus,
};
use crossterm::event::KeyCode;
use ratatui::DefaultTerminal;
use std::io;

fn main() -> io::Result<()> {
    ratatui::run(run_app)?;
    Ok(())
}

fn run_app(term: &mut DefaultTerminal) -> io::Result<()> {
    let mut app = create_app();

    'run_app: loop {
        term.draw(|frame| app.ui(frame))?;

        let event = crossterm::event::read()?;

        if event
            .as_key_press_event()
            .is_some_and(|e| matches!(e.code, KeyCode::Char('q' | 'Q')))
        {
            break 'run_app;
        }

        if let Some(ui_event) = UiEvent::parse_event(&event) {
            app.handle_ui(ui_event);
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
