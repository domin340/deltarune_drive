use crate::{
    app::App,
    conf::Bkp,
    manage_focus::Focus,
    my_widgets::button::{Button, ButtonState},
};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
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

    pub fn is_focus_explorer(&self) -> bool {
        match self.focus {
            Focus::ExplorerNew | Focus::ExplorerList => true,
            _ => false,
        }
    }

    pub fn is_focus_bkp(&self) -> bool {
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
            if self.is_focus_explorer() {
                block = block.border_style(Style::default().fg(Color::Blue));
            }

            block
        };

        let [explorer_list_area, _, explorer_new_button_area] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(3),
        ])
        .areas(explorer_block.inner(explorer_area));

        frame.render_widget(explorer_block, explorer_area);
        frame.render_stateful_widget(
            // here make the button secondary (find colors for the theme)
            Button::new(Line::from("New").centered()),
            explorer_new_button_area,
            &mut ButtonState::default().set_focused(self.is_focus(Focus::ExplorerNew)),
        );

        self.bkp_list(explorer_list_area, frame);

        // == handle right panel here ==
        let bkp_page_block = {
            let mut block = Block::bordered().title("Display (RIGHT)");
            if self.is_focus_bkp() {
                block = block.border_style(Style::default().fg(Color::Blue));
            }

            block
        };

        self.bkp_page(bkp_page_block.inner(bkp_page_area), frame);
        frame.render_widget(bkp_page_block, bkp_page_area);
    }

    fn bkp_list(&self, area: Rect, frame: &mut Frame) {
        let highlight_style = Style::default().bg(Color::DarkGray).fg(Color::White);
        frame.render_stateful_widget(
            List::new(self.bkp_names()).highlight_style(highlight_style),
            area,
            &mut ListState::default().with_selected(self.list_item_idx()),
        );
    }

    fn bkp_page(&self, area: Rect, frame: &mut Frame) {
        let [name_input_area, _] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);

        frame.render_widget(
            self.bkp_name_field
                .to_input_item(self.is_editing(Focus::BkpName))
                .block({
                    let mut block = Block::bordered().title("Backup Name");
                    if self.is_focus(Focus::BkpName) {
                        if self.editing {
                            block = block.border_style(Style::default().fg(Color::DarkGray));
                        } else {
                            block =
                                block.style(Style::default().bg(Color::DarkGray).fg(Color::White));
                        };
                    }

                    block
                }),
            name_input_area,
        );
    }

    fn bkp_names(&self) -> impl Iterator<Item = &str> {
        self.conf.bkps.iter().map(Bkp::name)
    }
}
