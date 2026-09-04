mod manage_focus;

use crate::{
    backups::Bkp,
    conf::Conf,
    my_widgets::button::{Button, ButtonState},
};
pub use manage_focus::{BkpPageFocus, ExplorerFocus, ExplorerListItem, Focus, UiAction};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, List, ListState},
};

pub struct State {
    /// stores all the backups and handles the IO inside the local data directory.
    pub conf: Conf,
    pub focus: Focus,
    pub list_item: Option<ExplorerListItem>,
}

impl State {
    fn is_explorer_focused(&self) -> bool {
        matches!(self.focus, Focus::Explorer(_))
    }

    fn is_bkp_page_focused(&self) -> bool {
        matches!(self.focus, Focus::BkpPage(_))
    }

    pub fn ui(&self, frame: &mut Frame) {
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
            &mut ListState::default().with_selected(self.list_item_idx()),
        );
    }

    fn bkp_names(&self) -> impl Iterator<Item = &str> {
        self.conf.bkps.iter().map(Bkp::name)
    }
}
