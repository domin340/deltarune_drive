use crate::{
    app::{App, MAX_NAME_FIELD_LINES},
    conf::Bkp,
    manage_focus::{ExplorerListItem, Focus},
    my_widgets::{
        button::{Button, ButtonState},
        input_field::FieldItem,
    },
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

        let content_bkp_page_area = bkp_page_block.inner(bkp_page_area);
        if let Some(bkp_idx) = self.list_item {
            self.concrete_bkp_page(bkp_idx, content_bkp_page_area, frame);
        } else {
            self.empty_bkp_page(content_bkp_page_area, frame);
        }

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

    fn empty_bkp_page(&self, area: Rect, frame: &mut Frame) {
        let [_, info_area, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(area);

        let line = Line::from("no backup selected").centered();
        frame.render_widget(line, info_area);
    }

    fn concrete_bkp_page(&self, idx: ExplorerListItem, area: Rect, frame: &mut Frame) {
        let [name_input_area, _] = Layout::vertical([
            Constraint::Length(MAX_NAME_FIELD_LINES as u16 + 2), /* name field + block */
            Constraint::Fill(1),
        ])
        .areas(area);

        let (name_field, name_cursor) = if self.is_editing(Focus::BkpName) {
            let cursor = self.bkp_name_field.cursor.clone();
            (self.bkp_name_field.to_input_item(), Some(cursor))
        } else {
            let bkp_name = self.selected_bkp().unwrap().name();
            (FieldItem::from_str(bkp_name), None)
        };

        frame.render_widget(
            name_field.set_cursor(name_cursor).block({
                let mut block = Block::bordered().title("Backup Name");
                if self.is_focus(Focus::BkpName) {
                    if self.editing {
                        block = block.border_style(Style::default().fg(Color::DarkGray));
                    } else {
                        block = block.style(Style::default().bg(Color::DarkGray).fg(Color::White));
                    };
                }

                block
            }),
            name_input_area,
        );
    }

    pub fn selected_bkp(&self) -> Option<&Bkp> {
        let idx = self.list_item_idx()?;
        self.conf.bkps.get(idx)
    }

    pub fn selected_bkp_mut(&mut self) -> Option<&mut Bkp> {
        let idx = self.list_item_idx()?;
        self.conf.bkps.get_mut(idx)
    }

    fn bkp_names(&self) -> impl Iterator<Item = &str> {
        self.conf.bkps.iter().map(Bkp::name)
    }
}
