use crate::app::App;
use crate::ui::Theme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &App, area: Rect, theme: &Theme) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.bg_secondary));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(20), // Logo
            Constraint::Min(20),    // System info
            Constraint::Length(40), // Quick stats
        ])
        .split(inner);

    // Logo/Title
    let logo = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(
                "SYSMON",
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled("v1.0", Style::default().fg(theme.fg_muted)),
        ]),
    ])
    .alignment(Alignment::Left);
    frame.render_widget(logo, chunks[0]);

    // System info
    let sys_info = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("  ", Style::default().fg(theme.accent)),
            Span::styled(&app.hostname, Style::default().fg(theme.fg)),
            Span::styled("  ", Style::default()),
            Span::styled(
                format!("{} {}", app.os_name, app.kernel_version),
                Style::default().fg(theme.fg_dim),
            ),
        ]),
        Line::from(vec![
            Span::styled("  ", Style::default().fg(theme.success)),
            Span::styled("Uptime ", Style::default().fg(theme.fg_muted)),
            Span::styled(
                app.format_uptime(),
                Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
            ),
        ]),
    ]);
    frame.render_widget(sys_info, chunks[1]);

    // Quick stats
    let cpu_color = theme.get_usage_color(app.cpu_data.total_usage);
    let mem_color = theme.get_usage_color(app.memory_data.used_percent);

    let stats = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("CPU ", Style::default().fg(theme.fg_muted)),
            Span::styled(
                format!("{:5.1}%", app.cpu_data.total_usage),
                Style::default().fg(cpu_color).add_modifier(Modifier::BOLD),
            ),
            Span::styled("   ", Style::default()),
            Span::styled("MEM ", Style::default().fg(theme.fg_muted)),
            Span::styled(
                format!("{:5.1}%", app.memory_data.used_percent),
                Style::default().fg(mem_color).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("PROC ", Style::default().fg(theme.fg_muted)),
            Span::styled(
                format!("{}", app.process_data.total_processes),
                Style::default().fg(theme.fg),
            ),
            Span::styled("  ", Style::default()),
            Span::styled("RUN ", Style::default().fg(theme.fg_muted)),
            Span::styled(
                format!("{}", app.process_data.running_processes),
                Style::default().fg(theme.success),
            ),
        ]),
    ])
    .alignment(Alignment::Right);
    frame.render_widget(stats, chunks[2]);
}
