use crate::app::App;
use crate::ui::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    symbols::Marker,
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &App, area: Rect, theme: &Theme) {
    let block = Block::default()
        .title(Line::from(vec![
            Span::styled(" ", Style::default()),
            Span::styled(
                "NETWORK",
                Style::default()
                    .fg(theme.net_down)
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
            Constraint::Length(2), // Stats
            Constraint::Min(3),    // Graph
        ])
        .split(inner);

    // Network stats
    let stats = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("↓ ", Style::default().fg(theme.net_down)),
            Span::styled(
                theme.format_speed(app.network_data.speed_down),
                Style::default()
                    .fg(theme.net_down)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("    ", Style::default()),
            Span::styled("↑ ", Style::default().fg(theme.net_up)),
            Span::styled(
                theme.format_speed(app.network_data.speed_up),
                Style::default()
                    .fg(theme.net_up)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Total: ", Style::default().fg(theme.fg_muted)),
            Span::styled("↓ ", Style::default().fg(theme.fg_dim)),
            Span::styled(
                theme.format_bytes(app.network_data.total_received),
                Style::default().fg(theme.fg_dim),
            ),
            Span::styled("  ↑ ", Style::default().fg(theme.fg_dim)),
            Span::styled(
                theme.format_bytes(app.network_data.total_transmitted),
                Style::default().fg(theme.fg_dim),
            ),
        ]),
    ]);
    frame.render_widget(stats, chunks[0]);

    // Graph
    let max_down = app
        .net_down_history
        .iter()
        .max()
        .copied()
        .unwrap_or(1024)
        .max(1024) as f64;
    let max_up = app
        .net_up_history
        .iter()
        .max()
        .copied()
        .unwrap_or(1024)
        .max(1024) as f64;
    let max_val = max_down.max(max_up);

    let down_data: Vec<(f64, f64)> = app
        .net_down_history
        .iter()
        .enumerate()
        .map(|(i, &v)| (i as f64, v as f64))
        .collect();

    let up_data: Vec<(f64, f64)> = app
        .net_up_history
        .iter()
        .enumerate()
        .map(|(i, &v)| (i as f64, v as f64))
        .collect();

    let datasets = vec![
        Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(theme.net_down))
            .data(&down_data),
        Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(theme.net_up))
            .data(&up_data),
    ];

    let chart = Chart::new(datasets)
        .x_axis(
            Axis::default()
                .bounds([0.0, app.net_down_history.len() as f64])
                .style(Style::default().fg(theme.border)),
        )
        .y_axis(
            Axis::default()
                .bounds([0.0, max_val])
                .style(Style::default().fg(theme.border)),
        )
        .style(Style::default().bg(theme.bg_secondary));

    frame.render_widget(chart, chunks[1]);
}
