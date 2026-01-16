use crate::app::App;
use crate::data::{BatteryData, BatteryState, TemperatureData};
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

    // Load Average
    let (l1, l5, l15) = app.load_avg;
    let load_color = if l1 > app.cpu_data.core_count as f64 {
        theme.usage_critical
    } else if l1 > app.cpu_data.core_count as f64 * 0.7 {
        theme.usage_high
    } else {
        theme.fg_dim
    };
    lines.push(Line::from(vec![
        Span::styled("Load ", Style::default().fg(theme.fg_muted)),
        Span::styled(
            format!("{:.2} {:.2} {:.2}", l1, l5, l15),
            Style::default().fg(load_color),
        ),
    ]));

    // Uptime
    lines.push(Line::from(vec![
        Span::styled("Up   ", Style::default().fg(theme.fg_muted)),
        Span::styled(app.format_uptime(), Style::default().fg(theme.fg_dim)),
    ]));

    // Battery (if available)
    if app.battery_data.has_battery {
        if let Some(battery) = app.battery_data.batteries.first() {
            let (icon, color) = match battery.state {
                BatteryState::Charging => ("⚡", theme.success),
                BatteryState::Discharging => {
                    if battery.percentage < 20.0 {
                        ("", theme.usage_critical)
                    } else if battery.percentage < 50.0 {
                        ("", theme.warning)
                    } else {
                        ("", theme.fg_dim)
                    }
                }
                BatteryState::Full => ("✓", theme.success),
                _ => ("", theme.fg_dim),
            };

            let time_str = match battery.state {
                BatteryState::Discharging => battery.time_to_empty
                    .map(|t| format!(" ({})", BatteryData::format_time(t)))
                    .unwrap_or_default(),
                BatteryState::Charging => battery.time_to_full
                    .map(|t| format!(" ({})", BatteryData::format_time(t)))
                    .unwrap_or_default(),
                _ => String::new(),
            };

            lines.push(Line::from(vec![
                Span::styled("Batt ", Style::default().fg(theme.fg_muted)),
                Span::styled(
                    format!("{} {:.0}%{}", icon, battery.percentage, time_str),
                    Style::default().fg(color),
                ),
            ]));
        }
    } else {
        // Show temperature if no battery
        if let Some(sensor) = app.temperature_data.sensors.first() {
            let temp_color = get_temp_color(sensor.temperature, sensor.critical, theme);
            lines.push(Line::from(vec![
                Span::styled("Temp ", Style::default().fg(theme.fg_muted)),
                Span::styled(
                    format!("{:.0}°C", sensor.temperature),
                    Style::default().fg(temp_color),
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
