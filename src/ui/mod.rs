mod cpu;
mod disk;
mod header;
mod memory;
mod network;
mod process;
mod system_info;
mod theme;

use crate::app::{App, KillSignal};
use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
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

    // Bottom row: Network | Disk | System Info
    let bottom_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(main_chunks[2]);

    network::draw(frame, app, bottom_row[0], &theme);
    disk::draw(frame, app, bottom_row[1], &theme);
    system_info::draw(frame, app, bottom_row[2], &theme);

    // Processes (full width)
    process::draw(frame, app, main_chunks[3], &theme);

    // Draw kill confirmation dialog if active
    if let Some((pid, name, signal)) = &app.kill_confirm {
        draw_kill_dialog(frame, *pid, name, *signal, &theme);
    }

    // Draw status message if present
    if let Some((message, _)) = &app.status_message {
        draw_status_message(frame, message, &theme);
    }
}

fn draw_kill_dialog(frame: &mut Frame, pid: u32, name: &str, signal: KillSignal, theme: &Theme) {
    let area = frame.area();

    // Calculate dialog position (centered)
    let dialog_width = 50u16.min(area.width.saturating_sub(4));
    let dialog_height = 7u16;
    let dialog_x = (area.width.saturating_sub(dialog_width)) / 2;
    let dialog_y = (area.height.saturating_sub(dialog_height)) / 2;

    let dialog_area = Rect::new(dialog_x, dialog_y, dialog_width, dialog_height);

    // Clear the area behind the dialog
    frame.render_widget(Clear, dialog_area);

    let signal_name = match signal {
        KillSignal::Term => "SIGTERM",
        KillSignal::Kill => "SIGKILL",
    };

    let signal_desc = match signal {
        KillSignal::Term => "(graceful)",
        KillSignal::Kill => "(force)",
    };

    let title_color = match signal {
        KillSignal::Term => theme.warning,
        KillSignal::Kill => theme.usage_critical,
    };

    let block = Block::default()
        .title(Line::from(vec![
            Span::styled(" ", Style::default()),
            Span::styled(
                format!("Kill Process - {}", signal_name),
                Style::default().fg(title_color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ", Style::default()),
        ]))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(title_color))
        .style(Style::default().bg(theme.bg_secondary));

    let truncated_name = if name.len() > 30 {
        format!("{}...", &name[..27])
    } else {
        name.to_string()
    };

    let content = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Kill ", Style::default().fg(theme.fg)),
            Span::styled(&truncated_name, Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            Span::styled(format!(" (PID {})?", pid), Style::default().fg(theme.fg)),
        ]),
        Line::from(vec![
            Span::styled(format!("  {} ", signal_name), Style::default().fg(title_color)),
            Span::styled(signal_desc, Style::default().fg(theme.fg_dim)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Press ", Style::default().fg(theme.fg_muted)),
            Span::styled("y", Style::default().fg(theme.success).add_modifier(Modifier::BOLD)),
            Span::styled(" to confirm, ", Style::default().fg(theme.fg_muted)),
            Span::styled("n", Style::default().fg(theme.usage_critical).add_modifier(Modifier::BOLD)),
            Span::styled(" to cancel", Style::default().fg(theme.fg_muted)),
        ]),
    ])
    .block(block);

    frame.render_widget(content, dialog_area);
}

fn draw_status_message(frame: &mut Frame, message: &str, theme: &Theme) {
    let area = frame.area();

    // Position at bottom center
    let msg_width = (message.len() as u16 + 4).min(area.width.saturating_sub(4));
    let msg_x = (area.width.saturating_sub(msg_width)) / 2;
    let msg_y = area.height.saturating_sub(3);

    let msg_area = Rect::new(msg_x, msg_y, msg_width, 3);

    frame.render_widget(Clear, msg_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.accent))
        .style(Style::default().bg(theme.bg_secondary));

    let content = Paragraph::new(Line::from(vec![
        Span::styled(message, Style::default().fg(theme.fg)),
    ]))
    .block(block);

    frame.render_widget(content, msg_area);
}
