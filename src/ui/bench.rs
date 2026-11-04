use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Paragraph, Widget},
};

use crate::{bench::BenchmarkState, config::Config};

pub fn render_bench(state: &BenchmarkState, config: &Config, area: Rect, buf: &mut Buffer) {
    let [top_center, bottom_keys] =
        Layout::vertical([Constraint::Length(5), Constraint::Min(3)]).areas(area);

    let ratio = if state.target_clicks > 0 {
        (state.current_clicks as f64 / state.target_clicks as f64).clamp(0.0, 1.0)
    } else {
        0.0
    };

    let status_line = if state.is_active {
        format!(
            "{}/{} clicks ({}%)",
            state.current_clicks,
            state.target_clicks,
            (ratio * 100.0) as u16
        )
    } else if state.current_clicks >= state.target_clicks && state.target_clicks > 0 {
        format!("Done! BPM: {}", state.result)
    } else {
        "Hit Enter to begin!".to_string()
    };

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(" Progress "))
        .gauge_style(
            Style::default()
                .fg(if ratio >= 1.0 {
                    Color::Green
                } else {
                    Color::Cyan
                })
                .bg(Color::DarkGray),
        )
        .ratio(ratio)
        .label(status_line);
    gauge.render(top_center, buf);

    let [k1_area, k2_area] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(bottom_keys);

    let k1_active = state.last_key == Some(config.key_primary);
    let k2_active = state.last_key == Some(config.key_secondary);

    let k1_title = format!(" Key 1 ({}) ", config.key_primary.to_ascii_uppercase());
    let k2_title = format!(" Key 2 ({}) ", config.key_secondary.to_ascii_uppercase());

    let k1_block = Block::default()
        .borders(Borders::ALL)
        .title(k1_title)
        .border_style(if k2_active {
            ratatui::style::Style::default().bold()
        } else {
            ratatui::style::Style::default()
        });

    let k2_block = Block::default()
        .borders(Borders::ALL)
        .title(k2_title)
        .border_style(if k2_active {
            ratatui::style::Style::default().bold()
        } else {
            ratatui::style::Style::default()
        });

    Paragraph::new(if k1_active { "Pressed" } else { "" })
        .centered()
        .block(k1_block)
        .render(k1_area, buf);

    Paragraph::new(if k2_active { "PRESSED" } else { "" })
        .centered()
        .block(k2_block)
        .render(k2_area, buf);
}
