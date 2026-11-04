use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

use crate::config::{BindState, Config};

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let [area] = Layout::vertical([Constraint::Percentage(percent_y)])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::horizontal([Constraint::Percentage(percent_x)])
        .flex(Flex::Center)
        .areas(area);
    area
}

pub fn render_rebind_popup(keybinds: &Config, area: Rect, buf: &mut Buffer) {
    let popup_area = centered_rect(45, 25, area);

    Clear.render(popup_area, buf);

    let prompt = match keybinds.state {
        BindState::WaitingFirst => "Press KEY 1 to bind",
        BindState::WaitingSecond => "Press KEY 2 to bind",
        BindState::Idle => return,
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Keybind Setup ")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().bold());

    Paragraph::new(prompt)
        .bold()
        .alignment(Alignment::Center)
        .block(block)
        .render(popup_area, buf)
}
