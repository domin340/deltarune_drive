use crate::{
    model::conf::Bkp,
    model::state::{Focus, State},
    my_widgets::button::{Button, ButtonState},
};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, List, ListState},
};

impl State {
    pub fn focus_on(&self, focus: Focus) -> bool {
        self.focus == focus
    }

    pub fn focus_on_explorer(&self) -> bool {
        match self.focus {
            Focus::Explorer | Focus::ExplorerNew | Focus::ExplorerList => true,
            _ => false,
        }
    }

    pub fn focus_on_bkp(&self) -> bool {
        match self.focus {
            Focus::BkpName
            | Focus::BkpDesc
            | Focus::BkpCreated
            | Focus::BkpUpdated
            | Focus::BkpDuplicate
            | Focus::BkpReplace
            | Focus::BkpDelete
            | Focus::BkpLoad => true,
            _ => false,
        }
    }

    pub fn ui(&self, frame: &mut Frame) {
        let [explorer_area, bkp_page_area] =
            Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)])
                .areas(frame.area());

        // == handle explorer here ==
        let explorer_block = {
            let mut block = Block::bordered().title("Explorer (LEFT)");
            if self.focus_on_explorer() {
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
            if self.focus_on_bkp() {
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
            &mut ListState::default().with_selected(self.list_item_idx()),
        );
    }

    fn bkp_names(&self) -> impl Iterator<Item = &str> {
        self.conf.bkps.iter().map(Bkp::name)
    }
}
