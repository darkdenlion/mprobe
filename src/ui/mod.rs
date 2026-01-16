mod cpu;
mod header;
mod memory;
mod network;
mod process;
mod system_info;
mod theme;

use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Margin},
    style::Style,
    widgets::Block,
    Frame,
};

pub use theme::Theme;

pub fn draw(frame: &mut Frame, app: &App) {
    let theme = Theme::default();

    // Draw background
    let bg_block = Block::default().style(Style::default().bg(theme.bg));
    frame.render_widget(bg_block, frame.area());

    // Main layout with margin for breathing room
    let outer_area = frame.area().inner(Margin {
        vertical: 0,
        horizontal: 1,
    });

    // Main vertical layout: Header | Widgets Grid | Processes
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),  // Header
            Constraint::Length(11), // Top row (CPU + Memory)
            Constraint::Length(9),  // Bottom row (Network + System)
            Constraint::Min(8),     // Processes
        ])
        .split(outer_area);

    // Draw header
    header::draw(frame, app, main_chunks[0], &theme);

    // Top row: CPU | Memory
    let top_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(main_chunks[1]);

    cpu::draw(frame, app, top_row[0], &theme);
    memory::draw(frame, app, top_row[1], &theme);

    // Bottom row: Network | System Info
    let bottom_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(main_chunks[2]);

    network::draw(frame, app, bottom_row[0], &theme);
    system_info::draw(frame, app, bottom_row[1], &theme);

    // Processes (full width)
    process::draw(frame, app, main_chunks[3], &theme);
}
