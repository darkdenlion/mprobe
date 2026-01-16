use crate::app::App;
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
                "SYSTEM",
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

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Min(3), // System info
            Constraint::Length(2), // Keyboard shortcuts
        ])
        .split(inner);

    // System information
    let cpu_name = if app.cpu_data.cpu_name.len() > 30 {
        format!("{}...", &app.cpu_data.cpu_name[..27])
    } else {
        app.cpu_data.cpu_name.clone()
    };

    let info = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("CPU  ", Style::default().fg(theme.fg_muted)),
            Span::styled(&cpu_name, Style::default().fg(theme.fg_dim)),
        ]),
        Line::from(vec![
            Span::styled("OS   ", Style::default().fg(theme.fg_muted)),
            Span::styled(
                format!("{} {}", app.os_name, app.kernel_version),
                Style::default().fg(theme.fg_dim),
            ),
        ]),
        Line::from(vec![
            Span::styled("Host ", Style::default().fg(theme.fg_muted)),
            Span::styled(&app.hostname, Style::default().fg(theme.fg_dim)),
        ]),
    ]);
    frame.render_widget(info, chunks[0]);

    // Keyboard shortcuts
    let shortcuts = Line::from(vec![
        Span::styled("q", Style::default().fg(theme.accent)),
        Span::styled(" quit ", Style::default().fg(theme.fg_muted)),
        Span::styled("/", Style::default().fg(theme.accent)),
        Span::styled(" filter ", Style::default().fg(theme.fg_muted)),
        Span::styled("s", Style::default().fg(theme.accent)),
        Span::styled(" sort ", Style::default().fg(theme.fg_muted)),
        Span::styled("r", Style::default().fg(theme.accent)),
        Span::styled(" reverse", Style::default().fg(theme.fg_muted)),
    ]);
    frame.render_widget(Paragraph::new(shortcuts), chunks[1]);
}
