mod bench;
mod header;
pub mod keybinds;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
};

use crate::app::App;

pub fn render(app: &App, area: Rect, buf: &mut Buffer) {
    let [header_area, content_area] =
        Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).areas(area);

    header::render_header(header_area, buf);
    bench::render_bench(&app.bench, &app.config, content_area, buf);

    if app.config.is_rebinding() {
        keybinds::render_rebind_popup(&app.config, area, buf);
    }
}
