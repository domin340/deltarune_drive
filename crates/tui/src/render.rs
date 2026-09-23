use crate::{
    app::{App, Popup},
    conf::Bkp,
    manage_focus::Focus,
    my_widgets::{
        button::{ButtonSimple, ButtonState},
        menu::{Menu, MenuItem, MenuState},
        popup::BinaryChoicePopup,
    },
};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, List, ListState},
};

impl App {
    pub fn is_focus(&self, focus: Focus) -> bool {
        self.focus == focus
    }

    pub fn is_editing(&self, focus: Focus) -> bool {
        self.editing && self.is_focus(focus)
    }

    pub fn is_explorer_focused(&self) -> bool {
        match self.focus {
            Focus::ExplorerNew | Focus::ExplorerList => true,
            _ => false,
        }
    }

    pub fn is_menu_focused(&self) -> bool {
        matches!(self.focus, Focus::Menu(_))
    }

    pub fn ui(&self, frame: &mut Frame) {
        let explorer_area = frame.area();

        // == handle explorer here ==
        let explorer_block = {
            let mut block = Block::bordered().title("Explorer (LEFT)");
            if self.is_explorer_focused() || self.is_menu_focused() {
                block = block.border_style(Style::default().fg(Color::Blue));
            }

            block
        };

        let [explorer_list_area, _, explorer_new_button_area] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .areas(explorer_block.inner(explorer_area));

        frame.render_widget(explorer_block, explorer_area);
        frame.render_stateful_widget(
            ButtonSimple::new(Line::from("New").centered())
                .set_min_x_pad(1)
                .focus_style(Style::default().bg(Color::DarkGray).fg(Color::White)),
            explorer_new_button_area,
            &mut ButtonState::default().set_focused(self.is_focus(Focus::ExplorerNew)),
        );

        let mut bkps_list_state = ListState::default().with_selected(self.list_item_idx());
        frame.render_stateful_widget(
            List::new(self.bkp_names())
                .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White)),
            explorer_list_area,
            &mut bkps_list_state,
        );

        if let Some(selected_menu_option) = self.focus.as_menu_index() {
            const MENU_WIDTH: u16 = 20;
            const MENU_HEIGHT: u16 = 4;
            const MENU_OFFSET_Y: u16 = 1;

            let selected_item = self
                .list_item
                .unwrap()
                .idx()
                .saturating_sub(bkps_list_state.offset());

            let item_x = explorer_list_area.x;
            let item_y = explorer_list_area.y + selected_item as u16 + MENU_OFFSET_Y;

            let screen = frame.area();
            let (x, y) = fit_on_screen(item_x, item_y, MENU_WIDTH, MENU_HEIGHT, screen);
            let menu_area = Rect::new(x, y, MENU_WIDTH, MENU_HEIGHT);

            frame.render_stateful_widget(
                Menu::new(
                    vec![
                        MenuItem::new("rename").with_key("<ctrl-r>"),
                        MenuItem::new("load").with_key("<ctrl-l>"),
                        MenuItem::new("clone").with_key("<ctrl-h>"),
                        MenuItem::new("delete").with_key("<ctrl-d>"),
                    ]
                    .into_iter(),
                )
                .with_item_key_gap(2)
                .with_selected_style(Style::default().add_modifier(Modifier::REVERSED)),
                menu_area,
                MenuState::default().with_selected(selected_menu_option),
            );
        }

        if let Some(popup) = &self.popup {
            let center_area = frame
                .area()
                .centered(Constraint::Percentage(50), Constraint::Percentage(50));

            match popup {
                Popup::NewBkp { choice } => {
                    frame.render_stateful_widget(
                        BinaryChoicePopup::new(
                            Line::from("create a new backup from deltarune files?").centered(),
                        ),
                        center_area,
                        &mut choice.clone(),
                    );
                }
                Popup::DeleteBkp { choice } => {
                    let selected_bkp_name = self.selected_bkp().name();
                    frame.render_stateful_widget(
                        BinaryChoicePopup::new(
                            Line::from(vec![
                                "do you want to delete this backup: ".into(),
                                selected_bkp_name.into(),
                            ])
                            .centered(),
                        ),
                        center_area,
                        &mut choice.clone(),
                    );
                }
            }
        }
    }

    pub fn get_bkp(&self, idx: usize) -> Option<&Bkp> {
        self.conf.bkps.get(idx)
    }

    pub fn selected_bkp(&self) -> &Bkp {
        self.get_bkp(self.list_item.unwrap().idx()).unwrap()
    }

    fn bkp_names(&self) -> impl Iterator<Item = &str> {
        self.conf.bkps.iter().map(Bkp::name)
    }
}

fn fit_on_screen(x: u16, y: u16, width: u16, height: u16, screen: Rect) -> (u16, u16) {
    let x = if x + width <= screen.right() {
        x
    } else {
        x.saturating_sub(width)
    };

    let y = if y + height <= screen.bottom() {
        y
    } else {
        y.saturating_sub(height)
    };

    let x = x.clamp(screen.left(), screen.right().saturating_sub(width));

    let y = y.clamp(screen.top(), screen.bottom().saturating_sub(height));

    (x, y)
}
