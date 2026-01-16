use crate::app::App;
use crate::data::DiskData;
use crate::ui::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &App, area: Rect, theme: &Theme) {
    let block = Block::default()
        .title(Line::from(vec![
            Span::styled(" ", Style::default()),
            Span::styled(
                "DISKS",
                Style::default()
                    .fg(theme.disk_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ", Style::default()),
        ]))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.bg_secondary));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if app.disk_data.disk_info.is_empty() {
        let no_disks = Paragraph::new("No disks found")
            .style(Style::default().fg(theme.fg_muted));
        frame.render_widget(no_disks, inner);
        return;
    }

    // Calculate how many disks we can show (2 lines per disk)
    let max_disks = (inner.height as usize).saturating_sub(1) / 2;
    let disks_to_show = app.disk_data.disk_info.iter().take(max_disks);

    let mut constraints: Vec<Constraint> = Vec::new();
    for _ in 0..max_disks.min(app.disk_data.disk_info.len()) {
        constraints.push(Constraint::Length(2));
    }
    constraints.push(Constraint::Min(0));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(constraints)
        .split(inner);

    for (i, disk) in disks_to_show.enumerate() {
        if i >= chunks.len() - 1 {
            break;
        }

        let disk_area = chunks[i];
        let usage_color = theme.get_usage_color(disk.used_percent);

        // Disk name and usage
        let mount_display = if disk.mount_point.len() > 15 {
            format!("...{}", &disk.mount_point[disk.mount_point.len() - 12..])
        } else {
            disk.mount_point.clone()
        };

        let header = Line::from(vec![
            Span::styled(&mount_display, Style::default().fg(theme.fg)),
            Span::styled(" ", Style::default()),
            Span::styled(
                format!("{:.0}%", disk.used_percent),
                Style::default().fg(usage_color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ", Style::default()),
            Span::styled(
                format!(
                    "{}/{}",
                    DiskData::format_bytes(disk.used),
                    DiskData::format_bytes(disk.total)
                ),
                Style::default().fg(theme.fg_dim),
            ),
        ]);

        // Progress bar
        let bar_width = disk_area.width.saturating_sub(2) as usize;
        let filled = ((disk.used_percent / 100.0) * bar_width as f64) as usize;
        let empty = bar_width.saturating_sub(filled);

        let bar = Line::from(vec![
            Span::styled("▓".repeat(filled), Style::default().fg(usage_color)),
            Span::styled("░".repeat(empty), Style::default().fg(theme.border)),
        ]);

        let disk_widget = Paragraph::new(vec![header, bar]);
        frame.render_widget(disk_widget, disk_area);
    }
}
