mod action;
mod app;
mod bench;
mod config;
mod ui;

use app::App;
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App::new();
    let app_result = app.run(&mut terminal);

    ratatui::restore();

    app_result
}
