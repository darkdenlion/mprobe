use crate::app::App;
use crate::data::TemperatureData;
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
            Constraint::Min(3), // System info + temps
            Constraint::Length(1), // Keyboard shortcuts hint
        ])
        .split(inner);

    // Build system info lines
    let mut lines: Vec<Line> = Vec::new();

    // Uptime
    lines.push(Line::from(vec![
        Span::styled("Up   ", Style::default().fg(theme.fg_muted)),
        Span::styled(app.format_uptime(), Style::default().fg(theme.fg_dim)),
    ]));

    // Host
    lines.push(Line::from(vec![
        Span::styled("Host ", Style::default().fg(theme.fg_muted)),
        Span::styled(&app.hostname, Style::default().fg(theme.fg_dim)),
    ]));

    // Temperature sensors (show top 2 if available)
    if !app.temperature_data.sensors.is_empty() {
        for sensor in app.temperature_data.sensors.iter().take(2) {
            let temp_color = get_temp_color(sensor.temperature, sensor.critical, theme);
            let label = if sensor.label.len() > 8 {
                format!("{}.", &sensor.label[..7])
            } else {
                format!("{:4}", sensor.label)
            };

            lines.push(Line::from(vec![
                Span::styled(format!("{} ", label), Style::default().fg(theme.fg_muted)),
                Span::styled(
                    format!("{:.0}°C", sensor.temperature),
                    Style::default().fg(temp_color).add_modifier(Modifier::BOLD),
                ),
            ]));
        }
    }

    let info = Paragraph::new(lines);
    frame.render_widget(info, chunks[0]);

    // Keyboard shortcuts hint
    let shortcuts = Line::from(vec![
        Span::styled("?", Style::default().fg(theme.accent)),
        Span::styled(" help", Style::default().fg(theme.fg_muted)),
    ]);
    frame.render_widget(Paragraph::new(shortcuts), chunks[1]);
}

fn get_temp_color(temp: f32, critical: Option<f32>, theme: &Theme) -> ratatui::style::Color {
    let idx = TemperatureData::get_temp_color_index(temp, critical);
    match idx {
        0 => theme.usage_low,
        1 => theme.usage_medium,
        2 => theme.usage_high,
        _ => theme.usage_critical,
    }
}
